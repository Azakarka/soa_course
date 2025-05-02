use std::time::Duration;

use kafka::client::RequiredAcks;

#[derive(Clone, Debug)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
    pub ack_timeout_duration: Duration,
    pub required_acks: RequiredAcks,
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub server_config: ServerConfig,
    pub postgres_config: deadpool_postgres::Config,
    pub kafka_config: KafkaConfig,
}

fn get_server_config() -> ServerConfig {
    let host = std::env::var("CONTENT_SERVICE_LAUNCH_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("CONTENT_SERVICE_PORT")
        .unwrap_or("8083".to_string())
        .parse::<u16>()
        .unwrap();

    ServerConfig { host, port }
}

fn get_postgres_config() -> deadpool_postgres::Config {
    let mut config = deadpool_postgres::Config::new();
    let host = std::env::var("POSTGRES_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("POSTGRES_PORT").unwrap_or("5432".to_string());
    let username = std::env::var("POSTGRES_USER").unwrap_or("user_service".to_string());
    let password = std::env::var("POSTGRES_PASSWORD").unwrap();
    let db_name = std::env::var("POSTGRES_DB").unwrap_or("user_db".to_string());

    config.host = Some(host);
    config.port = Some(port.parse::<u16>().unwrap());
    config.user = Some(username);
    config.password = Some(password);
    config.dbname = Some(db_name);

    config
}

fn get_kafka_config() -> KafkaConfig {
    let kafka_brokerke_host = std::env::var("KAFKA_BROKERS")
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    assert!(kafka_brokerke_host.len() > 0, "KAFKA_BROKERS is empty");

    KafkaConfig {
        brokers: kafka_brokerke_host,
        ack_timeout_duration: Duration::from_secs(1),
        required_acks: RequiredAcks::One,
    }
}

pub fn get_config() -> Config {
    Config {
        server_config: get_server_config(),
        postgres_config: get_postgres_config(),
        kafka_config: get_kafka_config(),
    }
}
