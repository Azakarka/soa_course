use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct KafkaPostViewEvent {
    pub post_id: String,
    pub user_id: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct KafkaPostLikeEvent {
    pub post_id: String,
    pub user_id: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct KafkaPostCommentEvent {
    pub post_id: String,
    pub user_id: String,
    pub created_at: i64,
}
