use keyring::Entry;

pub const ACTIVATION_SECURE_STORE_SERVICE_NAME: &str = "miro-fish-desktop-saas.activation";
pub const ACTIVATION_SECURE_STORE_LOOKUP_REFERENCE: &str = "device-bound-activation-token";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationSecureStoreStage {
    Materialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationSecureStoreSnapshot {
    pub stage: ActivationSecureStoreStage,
    pub service_name: &'static str,
    pub default_lookup_reference: &'static str,
    pub os_native_backend_selected: bool,
    pub secure_store_commit_implemented: bool,
    pub secure_store_retrieval_implemented: bool,
    pub secure_store_delete_implemented: bool,
    pub plaintext_fallback_allowed: bool,
    pub secure_store_handle_exposed_to_frontend: bool,
}

impl ActivationSecureStoreSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Activation secure-store surface is materialized. Stage is {:?}; service name is {}; default lookup reference is {}; OS-native backend selected is {}; secure-store commit implemented is {}; secure-store retrieval implemented is {}; secure-store delete implemented is {}; plaintext fallback allowed is {}; secure-store handle exposed to frontend is {}.",
            self.stage,
            self.service_name,
            self.default_lookup_reference,
            self.os_native_backend_selected,
            self.secure_store_commit_implemented,
            self.secure_store_retrieval_implemented,
            self.secure_store_delete_implemented,
            self.plaintext_fallback_allowed,
            self.secure_store_handle_exposed_to_frontend
        )
    }
}

pub fn activation_secure_store_surface() -> ActivationSecureStoreSnapshot {
    ActivationSecureStoreSnapshot {
        stage: ActivationSecureStoreStage::Materialized,
        service_name: ACTIVATION_SECURE_STORE_SERVICE_NAME,
        default_lookup_reference: ACTIVATION_SECURE_STORE_LOOKUP_REFERENCE,
        os_native_backend_selected: true,
        secure_store_commit_implemented: true,
        secure_store_retrieval_implemented: true,
        secure_store_delete_implemented: true,
        plaintext_fallback_allowed: false,
        secure_store_handle_exposed_to_frontend: false,
    }
}

pub trait ActivationSecureStoreBackend: Send + Sync {
    fn set_secret(&self, service_name: &str, lookup_reference: &str, token: &str) -> Result<(), String>;
    fn get_secret(
        &self,
        service_name: &str,
        lookup_reference: &str,
    ) -> Result<Option<String>, String>;
    fn delete_secret(&self, service_name: &str, lookup_reference: &str) -> Result<(), String>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct OsNativeActivationSecureStoreBackend;

impl OsNativeActivationSecureStoreBackend {
    fn entry(&self, service_name: &str, lookup_reference: &str) -> Result<Entry, String> {
        Entry::new(service_name, lookup_reference).map_err(|error| error.to_string())
    }
}

impl ActivationSecureStoreBackend for OsNativeActivationSecureStoreBackend {
    fn set_secret(&self, service_name: &str, lookup_reference: &str, token: &str) -> Result<(), String> {
        let entry = self.entry(service_name, lookup_reference)?;
        entry.set_password(token).map_err(|error| error.to_string())
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

pub struct ActivationSecureStore<B = OsNativeActivationSecureStoreBackend> {
    backend: B,
    service_name: &'static str,
    lookup_reference: String,
}

impl ActivationSecureStore<OsNativeActivationSecureStoreBackend> {
    pub fn os_native() -> Self {
        Self {
            backend: OsNativeActivationSecureStoreBackend,
            service_name: ACTIVATION_SECURE_STORE_SERVICE_NAME,
            lookup_reference: ACTIVATION_SECURE_STORE_LOOKUP_REFERENCE.to_string(),
        }
    }
}

impl<B: ActivationSecureStoreBackend> ActivationSecureStore<B> {
    #[cfg(test)]
    pub fn with_backend(backend: B, lookup_reference: impl Into<String>) -> Self {
        Self {
            backend,
            service_name: ACTIVATION_SECURE_STORE_SERVICE_NAME,
            lookup_reference: lookup_reference.into(),
        }
    }

    pub fn commit_token(&self, token: &str) -> Result<(), String> {
        self.backend
            .set_secret(self.service_name, &self.lookup_reference, token)
    }

    pub fn read_token(&self) -> Result<Option<String>, String> {
        self.backend
            .get_secret(self.service_name, &self.lookup_reference)
    }

    pub fn clear_token(&self) -> Result<(), String> {
        self.backend
            .delete_secret(self.service_name, &self.lookup_reference)
    }

    pub fn lookup_reference(&self) -> &str {
        &self.lookup_reference
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[derive(Clone, Default)]
    struct InMemorySecureStoreBackend {
        secret: Arc<Mutex<Option<String>>>,
    }

    impl ActivationSecureStoreBackend for InMemorySecureStoreBackend {
        fn set_secret(
            &self,
            _service_name: &str,
            _lookup_reference: &str,
            token: &str,
        ) -> Result<(), String> {
            *self.secret.lock().expect("lock poisoned") = Some(token.to_string());
            Ok(())
        }

        fn get_secret(
            &self,
            _service_name: &str,
            _lookup_reference: &str,
        ) -> Result<Option<String>, String> {
            Ok(self.secret.lock().expect("lock poisoned").clone())
        }

        fn delete_secret(&self, _service_name: &str, _lookup_reference: &str) -> Result<(), String> {
            *self.secret.lock().expect("lock poisoned") = None;
            Ok(())
        }
    }

    #[test]
    fn activation_secure_store_surface_keeps_frontend_away_from_secret_material() {
        let snapshot = activation_secure_store_surface();

        assert_eq!(snapshot.service_name, ACTIVATION_SECURE_STORE_SERVICE_NAME);
        assert_eq!(
            snapshot.default_lookup_reference,
            ACTIVATION_SECURE_STORE_LOOKUP_REFERENCE
        );
        assert!(snapshot.os_native_backend_selected);
        assert!(snapshot.secure_store_commit_implemented);
        assert!(snapshot.secure_store_retrieval_implemented);
        assert!(snapshot.secure_store_delete_implemented);
        assert!(!snapshot.plaintext_fallback_allowed);
        assert!(!snapshot.secure_store_handle_exposed_to_frontend);
    }

    #[test]
    fn activation_secure_store_round_trips_token_without_plaintext_fallback() {
        let backend = InMemorySecureStoreBackend::default();
        let store = ActivationSecureStore::with_backend(backend, "test-token");

        assert_eq!(store.read_token().expect("read should succeed"), None);

        store
            .commit_token("header.payload.signature")
            .expect("commit should succeed");

        assert_eq!(
            store.read_token().expect("read should succeed"),
            Some("header.payload.signature".to_string())
        );

        store.clear_token().expect("clear should succeed");
        assert_eq!(store.read_token().expect("read should succeed"), None);
    }
}