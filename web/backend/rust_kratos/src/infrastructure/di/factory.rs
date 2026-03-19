use crate::domain::ports::{
    inbound::{
        login::AuthenticationPort, recovery::RecoveryPort, registration::RegistrationPort,
        settings::SettingsPort, verification::VerificationPort,
    },
    outbound::{identity::IdentityPort, session::SessionPort},
};
use crate::infrastructure::adapters::cache::redis_cache::RedisCache;
use crate::infrastructure::adapters::db::pool::DbPool;
use crate::infrastructure::adapters::db::repositories::{
    auth::AuthRepository, identity::IdentityRepository, recovery::RecoveryRepository,
    registration::RegistrationRepository, session::SessionRepository, settings::SettingsRepository,
    verification::VerificationRepository,
};
use crate::infrastructure::di::adapter_factory::AdapterFactory;
use std::sync::Arc;

pub struct AdapterFactoryImpl {
    pool: DbPool,
    cache: RedisCache,
    cache_ttl_secs: u64,
}

impl AdapterFactoryImpl {
    pub fn new(pool: DbPool, cache: RedisCache, cache_ttl_secs: u64) -> Self {
        Self {
            pool,
            cache,
            cache_ttl_secs,
        }
    }
}

impl AdapterFactory for AdapterFactoryImpl {
    fn create_registration_adapter(&self) -> Arc<dyn RegistrationPort> {
        Arc::new(RegistrationRepository::new(self.pool.clone()))
    }
    fn create_authentication_adapter(&self) -> Arc<dyn AuthenticationPort> {
        Arc::new(AuthRepository::new(self.pool.clone()))
    }
    fn create_session_adapter(&self) -> Arc<dyn SessionPort> {
        Arc::new(SessionRepository::new(self.pool.clone()))
    }
    fn create_recovery_adapter(&self) -> Arc<dyn RecoveryPort> {
        Arc::new(RecoveryRepository::new(self.pool.clone()))
    }
    fn create_verification_adapter(&self) -> Arc<dyn VerificationPort> {
        Arc::new(VerificationRepository::new(self.pool.clone()))
    }
    fn create_identity_adapter(&self) -> Arc<dyn IdentityPort> {
        Arc::new(IdentityRepository::new(self.pool.clone()))
    }
    fn create_settings_adapter(&self) -> Arc<dyn SettingsPort> {
        Arc::new(SettingsRepository::new(self.pool.clone()))
    }
}
