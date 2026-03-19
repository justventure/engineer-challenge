use crate::infrastructure::adapters::db::schema::users;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub geo_location: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub geo_location: Option<String>,
    pub active: bool,
}
