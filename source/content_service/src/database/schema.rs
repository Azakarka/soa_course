use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PostgresMapper, Debug, Default)]
#[pg_mapper(table = "wall_posts")]
pub struct WallPostPostgres {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub post_id: Uuid,
    pub title: String,
    pub description: String,
    pub creator_id: Uuid,
    pub is_private: bool,
    pub tags: Vec<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}
