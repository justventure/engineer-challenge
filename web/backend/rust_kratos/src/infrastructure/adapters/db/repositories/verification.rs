use crate::domain::errors::DomainError;
use crate::domain::ports::inbound::verification::{
    SendCodeRequest, SubmitCodeRequest, VerificationPort, VerifyByLinkRequest,
};
use crate::infrastructure::adapters::db::pool::DbPool;
use async_trait::async_trait;

pub struct VerificationRepository {
    pool: DbPool,
}

impl VerificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VerificationPort for VerificationRepository {
    async fn verify_by_link(
        &self,
        _request: VerifyByLinkRequest,
        _cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        todo!()
    }

    async fn send_verification_code(
        &self,
        _request: SendCodeRequest,
        _cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        todo!()
    }

    async fn submit_verification_code(
        &self,
        _request: SubmitCodeRequest,
        _cookie: &str,
    ) -> Result<(), DomainError> {
        todo!()
    }
}
