use crate::domain::errors::DomainError;
use crate::domain::ports::inbound::login::{AuthenticationPort, LoginCredentials};
use crate::infrastructure::adapters::db::pool::DbPool;
use async_trait::async_trait;

pub struct AuthRepository {
    pool: DbPool,
}

impl AuthRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthenticationPort for AuthRepository {
    async fn initiate_login(&self, _cookie: Option<&str>) -> Result<String, DomainError> {
        todo!()
    }

    async fn complete_login(
        &self,
        _flow_id: &str,
        _credentials: LoginCredentials,
    ) -> Result<String, DomainError> {
        todo!()
    }
}
