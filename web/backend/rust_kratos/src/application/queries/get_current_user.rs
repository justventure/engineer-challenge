use crate::application::queries::QueryHandler;
use crate::domain::errors::{AuthError, DomainError};
use crate::domain::ports::outbound::identity::IdentityPort;
use crate::presentation::api::graphql::inputs::UserProfileOutput;
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetCurrentUserQuery {
    pub cookie: Option<String>,
}

pub struct GetCurrentUserQueryHandler {
    identity_port: Arc<dyn IdentityPort>,
}

impl GetCurrentUserQueryHandler {
    pub fn new(identity_port: Arc<dyn IdentityPort>) -> Self {
        Self { identity_port }
    }
}

#[async_trait]
impl QueryHandler<GetCurrentUserQuery, UserProfileOutput> for GetCurrentUserQueryHandler {
    async fn handle(&self, query: GetCurrentUserQuery) -> Result<UserProfileOutput, DomainError> {
        let cookie = query.cookie.ok_or(AuthError::NotAuthenticated)?;
        let profile = self.identity_port.get_current_user(&cookie).await?;
        Ok(profile.into())
    }
}
