use std::io::{Cursor, Read};
use std::path::Path;

use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use whatlang::{detect, Lang};
use zip::ZipArchive;

use crate::preflight_contract::{
    PreflightDocumentRejectionReason, SupportedSeedDocumentKind,
    MAX_SEED_DOCUMENT_SIZE_BYTES, MINIMUM_ENGLISH_CONFIDENCE_PERCENT,
    MINIMUM_EXTRACTABLE_TEXT_CHARACTERS,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SelectedSeedDocumentSnapshot {
    pub file_name: String,
    pub mime_type: Option<String>,
    pub detected_kind: Option<SupportedSeedDocumentKind>,
    pub size_bytes: u64,
    pub content_fingerprint_sha256: String,
    pub extracted_text_character_count: Option<usize>,
    pub english_confidence_percent: Option<u8>,
}

impl SelectedSeedDocumentSnapshot {
    fn from_boundary(
        file_name: &str,
        mime_type: Option<&str>,
        detected_kind: Option<SupportedSeedDocumentKind>,
        size_bytes: u64,
        bytes: &[u8],
    ) -> Self {
        Self {
            file_name: file_name.to_string(),
            mime_type: normalize_optional_text(mime_type),
            detected_kind,
            size_bytes,
            content_fingerprint_sha256: hex_sha256(bytes),
            extracted_text_character_count: None,
            english_confidence_percent: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedSeedDocument {
    pub snapshot: SelectedSeedDocumentSnapshot,
    pub supported_kind: SupportedSeedDocumentKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectedSeedDocument {
    pub snapshot: SelectedSeedDocumentSnapshot,
    pub rejection_reason: PreflightDocumentRejectionReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeedDocumentValidationDecision {
    Accepted(ValidatedSeedDocument),
    Rejected(RejectedSeedDocument),
}

pub fn validate_seed_document(
    file_name: &str,
    mime_type: Option<&str>,
    bytes: &[u8],
) -> SeedDocumentValidationDecision {
    let detected_kind = resolve_supported_seed_document_kind(file_name, mime_type);
    let mut snapshot = SelectedSeedDocumentSnapshot::from_boundary(
        file_name,
        mime_type,
        detected_kind,
        bytes.len() as u64,
        bytes,
    );

    let Some(supported_kind) = detected_kind else {
        return SeedDocumentValidationDecision::Rejected(RejectedSeedDocument {
            snapshot,
            rejection_reason: PreflightDocumentRejectionReason::UnsupportedFileType,
        });
    };

    if snapshot.size_bytes > MAX_SEED_DOCUMENT_SIZE_BYTES {
        return SeedDocumentValidationDecision::Rejected(RejectedSeedDocument {
            snapshot,
            rejection_reason: PreflightDocumentRejectionReason::FileTooLarge,
        });
    }

    let extracted_text = match extract_text_from_supported_document(supported_kind, bytes) {
        Ok(extracted_text) => normalize_extracted_text(&extracted_text),
        Err(()) => {
            return SeedDocumentValidationDecision::Rejected(RejectedSeedDocument {
                snapshot,
                rejection_reason: PreflightDocumentRejectionReason::TextExtractionFailed,
            })
        }
    };

    if extracted_text.is_empty() {
        return SeedDocumentValidationDecision::Rejected(RejectedSeedDocument {
            snapshot,
            rejection_reason: PreflightDocumentRejectionReason::TextExtractionFailed,
        });
    }

    let extracted_text_character_count = extracted_text.chars().count();
    snapshot.extracted_text_character_count = Some(extracted_text_character_count);

    if extracted_text_character_count < MINIMUM_EXTRACTABLE_TEXT_CHARACTERS {
        return SeedDocumentValidationDecision::Rejected(RejectedSeedDocument {
            snapshot,
            rejection_reason: PreflightDocumentRejectionReason::InsufficientExtractableText,
        });
    }

    let (is_english, english_confidence_percent) = detect_english_text(&extracted_text);
    snapshot.english_confidence_percent = Some(english_confidence_percent);

    if !is_english || english_confidence_percent < MINIMUM_ENGLISH_CONFIDENCE_PERCENT {
        return SeedDocumentValidationDecision::Rejected(RejectedSeedDocument {
            snapshot,
            rejection_reason: PreflightDocumentRejectionReason::NonEnglishSeedDocument,
        });
    }

    SeedDocumentValidationDecision::Accepted(ValidatedSeedDocument {
        snapshot,
        supported_kind,
    })
}

fn resolve_supported_seed_document_kind(
    file_name: &str,
    mime_type: Option<&str>,
) -> Option<SupportedSeedDocumentKind> {
    extension_supported_seed_document_kind(file_name)
        .or_else(|| mime_supported_seed_document_kind(mime_type))
}

fn extension_supported_seed_document_kind(file_name: &str) -> Option<SupportedSeedDocumentKind> {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|extension| extension.to_str())?
        .trim()
        .to_ascii_lowercase();

    match extension.as_str() {
        "pdf" => Some(SupportedSeedDocumentKind::Pdf),
        "docx" => Some(SupportedSeedDocumentKind::Docx),
        "txt" => Some(SupportedSeedDocumentKind::Txt),
        "md" | "markdown" => Some(SupportedSeedDocumentKind::Markdown),
        _ => None,
    }
}

fn mime_supported_seed_document_kind(mime_type: Option<&str>) -> Option<SupportedSeedDocumentKind> {
    let normalized_mime_type = normalize_optional_text(mime_type)?;

    match normalized_mime_type.as_str() {
        "application/pdf" => Some(SupportedSeedDocumentKind::Pdf),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            Some(SupportedSeedDocumentKind::Docx)
        }
        "text/plain" => Some(SupportedSeedDocumentKind::Txt),
        "text/markdown" | "text/x-markdown" => Some(SupportedSeedDocumentKind::Markdown),
        _ => None,
    }
}

fn extract_text_from_supported_document(
    supported_kind: SupportedSeedDocumentKind,
    bytes: &[u8],
) -> Result<String, ()> {
    match supported_kind {
        SupportedSeedDocumentKind::Txt | SupportedSeedDocumentKind::Markdown => {
            Ok(String::from_utf8_lossy(bytes).into_owned())
        }
        SupportedSeedDocumentKind::Docx => extract_docx_text(bytes),
        SupportedSeedDocumentKind::Pdf => extract_pdf_text(bytes),
    }
}

fn extract_docx_text(bytes: &[u8]) -> Result<String, ()> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|_| ())?;
    let mut document_xml = archive.by_name("word/document.xml").map_err(|_| ())?;
    let mut xml = String::new();

    document_xml.read_to_string(&mut xml).map_err(|_| ())?;

    Ok(extract_wordprocessingml_text(&xml))
}

fn extract_wordprocessingml_text(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut output = String::new();
    let mut buffer = Vec::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Text(text)) => {
                output.push_str(&String::from_utf8_lossy(text.as_ref()));
            }
            Ok(Event::CData(text)) => {
                output.push_str(&String::from_utf8_lossy(text.as_ref()));
            }
            Ok(Event::Empty(element)) => match element.name().as_ref() {
                b"w:tab" => output.push('\t'),
                b"w:br" | b"w:cr" => output.push('\n'),
                _ => {}
            },
            Ok(Event::End(element)) if element.name().as_ref() == b"w:p" => {
                output.push('\n');
            }
            Ok(Event::Eof) => break,
            Err(_) => return String::new(),
            _ => {}
        }

        buffer.clear();
    }

    output
}

fn extract_pdf_text(bytes: &[u8]) -> Result<String, ()> {
    pdf_extract::extract_text_from_mem(bytes).map_err(|_| ())
}

fn normalize_extracted_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn detect_english_text(text: &str) -> (bool, u8) {
    let Some(info) = detect(text) else {
        return (false, 0);
    };

    let english_confidence_percent = confidence_percent(info.confidence());
    (info.lang() == Lang::Eng, english_confidence_percent)
}

fn confidence_percent(confidence: f64) -> u8 {
    let bounded_confidence = confidence.clamp(0.0, 1.0);
    (bounded_confidence * 100.0).round() as u8
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_ascii_lowercase())
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);

    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn english_text() -> String {
        "This is a governed English seed document used to exercise deterministic shell validation. "
            .repeat(20)
    }

    fn spanish_text() -> String {
        "Este es un documento de entrada en espanol que debe ser rechazado por la validacion de idioma. "
            .repeat(20)
    }

    fn build_docx_bytes(text: &str) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(&mut cursor);
        let options = zip::write::SimpleFileOptions::default();
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
              <w:body>
                <w:p><w:r><w:t>{text}</w:t></w:r></w:p>
              </w:body>
            </w:document>"#
        );

        writer.start_file("word/document.xml", options).unwrap();
        writer.write_all(xml.as_bytes()).unwrap();
        writer.finish().unwrap();

        cursor.into_inner()
    }

    #[test]
    fn txt_document_can_pass_validation() {
        let bytes = english_text().into_bytes();
        let outcome = validate_seed_document("seed.txt", Some("text/plain"), &bytes);

        match outcome {
            SeedDocumentValidationDecision::Accepted(validated) => {
                assert_eq!(validated.supported_kind, SupportedSeedDocumentKind::Txt);
                assert_eq!(validated.snapshot.detected_kind, Some(SupportedSeedDocumentKind::Txt));
                assert!(validated.snapshot.extracted_text_character_count.unwrap() >= 1_000);
                assert!(validated.snapshot.english_confidence_percent.unwrap() >= 80);
            }
            SeedDocumentValidationDecision::Rejected(rejected) => {
                panic!("expected accepted txt document, got {:?}", rejected.rejection_reason);
            }
        }
    }

    #[test]
    fn docx_document_can_pass_validation() {
        let bytes = build_docx_bytes(&english_text());
        let outcome = validate_seed_document(
            "seed.docx",
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
            &bytes,
        );

        match outcome {
            SeedDocumentValidationDecision::Accepted(validated) => {
                assert_eq!(validated.supported_kind, SupportedSeedDocumentKind::Docx);
                assert_eq!(validated.snapshot.detected_kind, Some(SupportedSeedDocumentKind::Docx));
                assert!(validated.snapshot.extracted_text_character_count.unwrap() >= 1_000);
            }
            SeedDocumentValidationDecision::Rejected(rejected) => {
                panic!("expected accepted docx document, got {:?}", rejected.rejection_reason);
            }
        }
    }

    #[test]
    fn unsupported_extension_is_rejected() {
        let bytes = english_text().into_bytes();
        let outcome = validate_seed_document("seed.csv", Some("text/csv"), &bytes);

        match outcome {
            SeedDocumentValidationDecision::Accepted(_) => {
                panic!("expected unsupported file type rejection");
            }
            SeedDocumentValidationDecision::Rejected(rejected) => {
                assert_eq!(
                    rejected.rejection_reason,
                    PreflightDocumentRejectionReason::UnsupportedFileType
                );
                assert_eq!(rejected.snapshot.detected_kind, None);
            }
        }
    }

    #[test]
    fn oversized_document_is_rejected() {
        let bytes = vec![b'a'; (MAX_SEED_DOCUMENT_SIZE_BYTES as usize) + 1];
        let outcome = validate_seed_document("seed.txt", Some("text/plain"), &bytes);

        match outcome {
            SeedDocumentValidationDecision::Accepted(_) => {
                panic!("expected file-too-large rejection");
            }
            SeedDocumentValidationDecision::Rejected(rejected) => {
                assert_eq!(
                    rejected.rejection_reason,
                    PreflightDocumentRejectionReason::FileTooLarge
                );
            }
        }
    }

    #[test]
    fn corrupt_docx_is_rejected_as_text_extraction_failure() {
        let bytes = vec![0, 1, 2, 3, 4, 5];
        let outcome = validate_seed_document(
            "seed.docx",
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
            &bytes,
        );

        match outcome {
            SeedDocumentValidationDecision::Accepted(_) => {
                panic!("expected text extraction failure rejection");
            }
            SeedDocumentValidationDecision::Rejected(rejected) => {
                assert_eq!(
                    rejected.rejection_reason,
                    PreflightDocumentRejectionReason::TextExtractionFailed
                );
            }
        }
    }

    #[test]
    fn short_extractable_text_is_rejected() {
        let bytes = b"short english text".to_vec();
        let outcome = validate_seed_document("seed.txt", Some("text/plain"), &bytes);

        match outcome {
            SeedDocumentValidationDecision::Accepted(_) => {
                panic!("expected insufficient text rejection");
            }
            SeedDocumentValidationDecision::Rejected(rejected) => {
                assert_eq!(
                    rejected.rejection_reason,
                    PreflightDocumentRejectionReason::InsufficientExtractableText
                );
            }
        }
    }

    #[test]
    fn non_english_text_is_rejected() {
        let bytes = spanish_text().into_bytes();
        let outcome = validate_seed_document("seed.txt", Some("text/plain"), &bytes);

        match outcome {
            SeedDocumentValidationDecision::Accepted(_) => {
                panic!("expected non-English rejection");
            }
            SeedDocumentValidationDecision::Rejected(rejected) => {
                assert_eq!(
                    rejected.rejection_reason,
                    PreflightDocumentRejectionReason::NonEnglishSeedDocument
                );
                assert!(rejected.snapshot.english_confidence_percent.is_some());
            }
        }
    }
}