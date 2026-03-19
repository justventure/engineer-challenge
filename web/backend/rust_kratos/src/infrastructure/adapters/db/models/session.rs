use crate::infrastructure::adapters::db::schema::sessions;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub is_recovery: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub is_recovery: bool,
    pub expires_at: DateTime<Utc>,
}
