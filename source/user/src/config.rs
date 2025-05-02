#[derive(Clone, Debug, Default)]
pub struct UserServiceConfig {
    pub postgres_config: deadpool_postgres::Config,
    pub public_key: String,
    pub private_key: String,
    pub token_duration: u64,
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

pub fn get_config() -> UserServiceConfig {
    let public_key = include_str!("../cert/public.pem").to_string();
    let private_key = include_str!("../cert/private.pem").to_string();
    let token_duration = 3600;

    UserServiceConfig {
        postgres_config: get_postgres_config(),
        public_key,
        private_key,
        token_duration,
    }
}
