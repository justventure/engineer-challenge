use crate::domain::errors::DomainError;
use crate::domain::ports::inbound::registration::{RegistrationData, RegistrationPort};
use crate::infrastructure::adapters::db::pool::DbPool;
use async_trait::async_trait;

pub struct RegistrationRepository {
    pool: DbPool,
}

impl RegistrationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegistrationPort for RegistrationRepository {
    async fn initiate_registration(&self, _cookie: Option<&str>) -> Result<String, DomainError> {
        todo!()
    }

    async fn complete_registration(
        &self,
        _flow_id: &str,
        _data: RegistrationData,
    ) -> Result<String, DomainError> {
        todo!()
    }
}
