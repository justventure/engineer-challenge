use crate::infrastructure::adapters::db::schema::settings_audit;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = settings_audit)]
pub struct SettingsAudit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub method: String,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = settings_audit)]
pub struct NewSettingsAudit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub method: String,
    pub ip_address: Option<String>,
}
