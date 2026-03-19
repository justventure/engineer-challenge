use crate::domain::errors::DomainError;
use crate::domain::ports::inbound::settings::{SettingsData, SettingsPort};
use crate::infrastructure::adapters::db::pool::DbPool;
use async_trait::async_trait;

pub struct SettingsRepository {
    pool: DbPool,
}

impl SettingsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsPort for SettingsRepository {
    async fn initiate_settings(&self, _cookie: &str) -> Result<String, DomainError> {
        todo!()
    }

    async fn update_settings(
        &self,
        _flow_id: &str,
        _data: SettingsData,
        _cookie: &str,
    ) -> Result<(String, Vec<String>), DomainError> {
        todo!()
    }
}
