use crate::infrastructure::adapters::db::schema::verification_tokens;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = verification_tokens)]
pub struct VerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub code: Option<String>,
    pub purpose: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = verification_tokens)]
pub struct NewVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub code: Option<String>,
    pub purpose: String,
    pub expires_at: DateTime<Utc>,
}
