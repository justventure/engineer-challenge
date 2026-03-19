use crate::domain::errors::DomainError;
use crate::domain::ports::inbound::recovery::{RecoveryPort, RecoveryRequest};
use crate::infrastructure::adapters::db::pool::DbPool;
use async_trait::async_trait;

pub struct RecoveryRepository {
    pool: DbPool,
}

impl RecoveryRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecoveryPort for RecoveryRepository {
    async fn initiate_recovery(
        &self,
        _request: RecoveryRequest,
        _cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        todo!()
    }
}
