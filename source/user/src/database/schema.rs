use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "user_profiles")]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_uuid: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub birth_date: Option<chrono::DateTime<Utc>>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>
}
