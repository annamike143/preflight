use keyring::Entry;

use crate::provider_configuration_contract::{SupportedProvider, SUPPORTED_PROVIDERS};

pub const PROVIDER_SECURE_STORE_DOMAIN_NAME: &str = "OsNativeSecureStorageDomain";
pub const PROVIDER_SECURE_STORE_ABSTRACTION_NAME: &str = "ShellOwnedProviderSecureStore";
pub const PROVIDER_SECURE_STORE_SERVICE_NAME: &str = "miro-fish-desktop-saas.provider";
pub const PROVIDER_SECRET_LOOKUP_REFERENCE_PREFIX: &str = "provider-api-key";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderSecureStoreStage {
    Materialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderSecureStoreSnapshot {
    pub stage: ProviderSecureStoreStage,
    pub secure_store_domain_name: &'static str,
    pub secure_store_abstraction_name: &'static str,
    pub service_name: &'static str,
    pub supported_provider_count: usize,
    pub os_native_backend_selected: bool,
    pub secure_store_commit_implemented: bool,
    pub secure_store_retrieval_implemented: bool,
    pub secure_store_delete_implemented: bool,
    pub plaintext_fallback_allowed: bool,
    pub secure_store_handle_exposed_to_frontend: bool,
}

pub fn provider_secure_store_surface() -> ProviderSecureStoreSnapshot {
    ProviderSecureStoreSnapshot {
        stage: ProviderSecureStoreStage::Materialized,
        secure_store_domain_name: PROVIDER_SECURE_STORE_DOMAIN_NAME,
        secure_store_abstraction_name: PROVIDER_SECURE_STORE_ABSTRACTION_NAME,
        service_name: PROVIDER_SECURE_STORE_SERVICE_NAME,
        supported_provider_count: SUPPORTED_PROVIDERS.len(),
        os_native_backend_selected: true,
        secure_store_commit_implemented: true,
        secure_store_retrieval_implemented: true,
        secure_store_delete_implemented: true,
        plaintext_fallback_allowed: false,
        secure_store_handle_exposed_to_frontend: false,
    }
}

pub trait ProviderSecureStoreBackend: Send + Sync {
    fn set_secret(
        &self,
        service_name: &str,
        lookup_reference: &str,
        secret: &str,
    ) -> Result<(), String>;
    fn get_secret(
        &self,
        service_name: &str,
        lookup_reference: &str,
    ) -> Result<Option<String>, String>;
    fn delete_secret(&self, service_name: &str, lookup_reference: &str) -> Result<(), String>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct OsNativeProviderSecureStoreBackend;

impl OsNativeProviderSecureStoreBackend {
    fn entry(&self, service_name: &str, lookup_reference: &str) -> Result<Entry, String> {
        Entry::new(service_name, lookup_reference).map_err(|error| error.to_string())
    }
}

impl ProviderSecureStoreBackend for OsNativeProviderSecureStoreBackend {
    fn set_secret(
        &self,
        service_name: &str,
        lookup_reference: &str,
        secret: &str,
    ) -> Result<(), String> {
        let entry = self.entry(service_name, lookup_reference)?;
        entry.set_password(secret).map_err(|error| error.to_string())
    }

    fn get_secret(
        &self,
        service_name: &str,
        lookup_reference: &str,
    ) -> Result<Option<String>, String> {
        let entry = self.entry(service_name, lookup_reference)?;
        match entry.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }

    fn delete_secret(&self, service_name: &str, lookup_reference: &str) -> Result<(), String> {
        let entry = self.entry(service_name, lookup_reference)?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

pub struct ProviderSecureStore<B = OsNativeProviderSecureStoreBackend> {
    backend: B,
    service_name: &'static str,
}

impl ProviderSecureStore<OsNativeProviderSecureStoreBackend> {
    pub fn os_native() -> Self {
        Self {
            backend: OsNativeProviderSecureStoreBackend,
            service_name: PROVIDER_SECURE_STORE_SERVICE_NAME,
        }
    }
}

impl<B: ProviderSecureStoreBackend> ProviderSecureStore<B> {
    #[cfg(test)]
    pub fn with_backend(backend: B) -> Self {
        Self {
            backend,
            service_name: PROVIDER_SECURE_STORE_SERVICE_NAME,
        }
    }

    pub fn lookup_reference(&self, provider: SupportedProvider) -> String {
        format!(
            "{PROVIDER_SECRET_LOOKUP_REFERENCE_PREFIX}.{}",
            provider.lookup_key()
        )
    }

    pub fn commit_secret(&self, provider: SupportedProvider, secret: &str) -> Result<(), String> {
        self.backend.set_secret(
            self.service_name,
            &self.lookup_reference(provider),
            secret,
        )
    }

    pub fn read_secret(&self, provider: SupportedProvider) -> Result<Option<String>, String> {
        self.backend.get_secret(
            self.service_name,
            &self.lookup_reference(provider),
        )
    }

    pub fn clear_secret(&self, provider: SupportedProvider) -> Result<(), String> {
        self.backend.delete_secret(
            self.service_name,
            &self.lookup_reference(provider),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use super::*;

    #[derive(Clone, Default)]
    struct InMemorySecureStoreBackend {
        secrets: Arc<Mutex<HashMap<String, String>>>,
    }

    impl ProviderSecureStoreBackend for InMemorySecureStoreBackend {
        fn set_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
            secret: &str,
        ) -> Result<(), String> {
            self.secrets
                .lock()
                .expect("lock poisoned")
                .insert(lookup_reference.to_string(), secret.to_string());
            Ok(())
        }

        fn get_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
        ) -> Result<Option<String>, String> {
            Ok(self
                .secrets
                .lock()
                .expect("lock poisoned")
                .get(lookup_reference)
                .cloned())
        }

        fn delete_secret(
            &self,
            _service_name: &str,
            lookup_reference: &str,
        ) -> Result<(), String> {
            self.secrets
                .lock()
                .expect("lock poisoned")
                .remove(lookup_reference);
            Ok(())
        }
    }

    #[test]
    fn provider_secure_store_surface_keeps_secret_handles_inside_the_shell() {
        let snapshot = provider_secure_store_surface();

        assert_eq!(snapshot.secure_store_domain_name, PROVIDER_SECURE_STORE_DOMAIN_NAME);
        assert_eq!(
            snapshot.secure_store_abstraction_name,
            PROVIDER_SECURE_STORE_ABSTRACTION_NAME
        );
        assert_eq!(snapshot.service_name, PROVIDER_SECURE_STORE_SERVICE_NAME);
        assert_eq!(snapshot.supported_provider_count, SUPPORTED_PROVIDERS.len());
        assert!(snapshot.os_native_backend_selected);
        assert!(snapshot.secure_store_commit_implemented);
        assert!(snapshot.secure_store_retrieval_implemented);
        assert!(snapshot.secure_store_delete_implemented);
        assert!(!snapshot.plaintext_fallback_allowed);
        assert!(!snapshot.secure_store_handle_exposed_to_frontend);
    }

    #[test]
    fn provider_secure_store_round_trips_secrets_per_supported_provider() {
        let store = ProviderSecureStore::with_backend(InMemorySecureStoreBackend::default());

        assert_eq!(store.read_secret(SupportedProvider::OpenAi).unwrap(), None);
        assert_eq!(store.read_secret(SupportedProvider::Google).unwrap(), None);

        store
            .commit_secret(SupportedProvider::OpenAi, "sk-openai-001")
            .expect("OpenAI secret should commit");
        store
            .commit_secret(SupportedProvider::Google, "google-secret-001")
            .expect("Google secret should commit");

        assert_eq!(
            store.read_secret(SupportedProvider::OpenAi).unwrap(),
            Some("sk-openai-001".to_string())
        );
        assert_eq!(
            store.read_secret(SupportedProvider::Google).unwrap(),
            Some("google-secret-001".to_string())
        );

        store
            .clear_secret(SupportedProvider::OpenAi)
            .expect("OpenAI secret should clear");
        store
            .clear_secret(SupportedProvider::Google)
            .expect("Google secret should clear");

        assert_eq!(store.read_secret(SupportedProvider::OpenAi).unwrap(), None);
        assert_eq!(store.read_secret(SupportedProvider::Google).unwrap(), None);
    }
}