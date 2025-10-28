use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub user_tag: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub timezone: Option<String>,
    pub avatar_url: Option<String>,
    pub google_sub: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
