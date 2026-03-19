use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::DomainError;
use crate::domain::ports::outbound::identity::IdentityPort;
use crate::infrastructure::adapters::db::models::session::Session;
use crate::infrastructure::adapters::db::models::user::User;
use crate::infrastructure::adapters::db::pool::DbPool;
use crate::infrastructure::adapters::db::schema::{sessions, users};
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub struct IdentityRepository {
    pool: DbPool,
}

impl IdentityRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IdentityPort for IdentityRepository {
    async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, DomainError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| DomainError::InternalError(format!("DB connection error: {e}")))?;

        let session: Session = sessions::table
            .filter(sessions::token.eq(cookie))
            .filter(sessions::expires_at.gt(Utc::now()))
            .filter(sessions::is_recovery.eq(false))
            .select(Session::as_select())
            .first(&mut conn)
            .await
            .map_err(|_| DomainError::Unauthorized("Session not found or expired".into()))?;

        let user: User = users::table
            .filter(users::id.eq(session.user_id))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|_| DomainError::NotFound("User not found".into()))?;

        Ok(UserProfile {
            id: user.id.to_string(),
            email: user.email,
            username: user.username,
            geo_location: user.geo_location,
            created_at: Some(user.created_at),
            updated_at: Some(user.updated_at),
            state: None,
            active: user.active,
            expires_at: Some(session.expires_at),
        })
    }
}
