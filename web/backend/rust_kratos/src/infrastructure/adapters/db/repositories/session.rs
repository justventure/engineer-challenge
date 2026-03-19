use crate::domain::errors::DomainError;
use crate::domain::ports::outbound::session::SessionPort;
use crate::infrastructure::adapters::db::pool::DbPool;
use crate::infrastructure::adapters::db::schema::sessions;
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub struct SessionRepository {
    pool: DbPool,
}

impl SessionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionPort for SessionRepository {
    async fn logout(&self, cookie: &str) -> Result<(), DomainError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| DomainError::InternalError(format!("DB connection error: {e}")))?;

        diesel::delete(sessions::table.filter(sessions::token.eq(cookie)))
            .execute(&mut conn)
            .await
            .map_err(|e| DomainError::InternalError(format!("Logout error: {e}")))?;

        Ok(())
    }

    async fn check_active_session(&self, cookie: Option<&str>) -> bool {
        let Some(token) = cookie else { return false };
        let Ok(mut conn) = self.pool.get().await else {
            return false;
        };

        sessions::table
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(Utc::now()))
            .filter(sessions::is_recovery.eq(false))
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map(|c| c > 0)
            .unwrap_or(false)
    }

    async fn is_recovery_session(&self, cookie: Option<&str>) -> bool {
        let Some(token) = cookie else { return false };
        let Ok(mut conn) = self.pool.get().await else {
            return false;
        };

        sessions::table
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(Utc::now()))
            .filter(sessions::is_recovery.eq(true))
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map(|c| c > 0)
            .unwrap_or(false)
    }
}
