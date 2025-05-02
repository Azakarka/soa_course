use kafka::producer::{Producer, Record};

use crate::config::Config;

use super::schema::{KafkaPostCommentEvent, KafkaPostLikeEvent, KafkaPostViewEvent};

fn do_produce(
    topic: &str,
    value: String,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut producer = Producer::from_hosts(config.kafka_config.brokers.clone())
        .with_ack_timeout(config.kafka_config.ack_timeout_duration)
        .with_required_acks(config.kafka_config.required_acks)
        .create()?;
    producer.send(&Record::from_value(topic, value))?;
    Ok(())
}

pub fn produce_post_view_event(
    post_id: String,
    user_id: String,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let kafka_event = KafkaPostViewEvent {
        post_id,
        user_id,
        created_at: chrono::Utc::now().timestamp(),
    };
    do_produce("content_view", serde_json::to_string(&kafka_event)?, config)
}

pub fn produce_post_like_event(
    post_id: String,
    user_id: String,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let kafka_event = KafkaPostLikeEvent {
        post_id,
        user_id,
        created_at: chrono::Utc::now().timestamp(),
    };
    do_produce("content_like", serde_json::to_string(&kafka_event)?, config)
}

pub fn produce_post_comment_event(
    post_id: String,
    user_id: String,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let kafka_event = KafkaPostCommentEvent {
        post_id,
        user_id,
        created_at: chrono::Utc::now().timestamp(),
    };
    do_produce(
        "content_comment",
        serde_json::to_string(&kafka_event)?,
        config,
    )
}
