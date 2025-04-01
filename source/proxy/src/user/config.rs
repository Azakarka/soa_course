use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct ProxyConfig {
    pub user_service_host: String,
    pub user_service_port: u16,
    pub user_service_url: String,
    pub public_key: String,
    pub content_service_host: String,
    pub content_service_url: String,
    pub content_service_port: u16,
}

#[derive(Deserialize)]
struct GetConfigResp {
    public_key: String,
}

pub async fn get_config() -> ProxyConfig {
    let user_service_host = std::env::var("USER_SERVICE_HOST").unwrap_or("0.0.0.0".to_string());
    let user_service_port = std::env::var("USER_SERVICE_PORT").unwrap_or("8082".to_string());
    let user_service_port = user_service_port.parse::<u16>().unwrap();
    let user_service_url =
        format!("http://{}:{}", user_service_host.clone(), user_service_port).to_string();
    println!("User service url: {}", user_service_url);

    let content_service_host = std::env::var("CONTENT_SERVICE_HOST").unwrap_or("0.0.0.0".to_string());
    let content_service_port = std::env::var("CONTENT_SERVICE_PORT").unwrap_or("8083".to_string());
    let content_service_port = content_service_port.parse::<u16>().unwrap();
    let content_service_url =
        format!("http://{}:{}", content_service_host.clone(), content_service_port).to_string();
    println!("Content service url: {}", content_service_url);

    let public_key = reqwest::get(user_service_url.clone() + "/get_public_key")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("Received public key: {}", public_key);
    let public_key: GetConfigResp = serde_json::from_str(&public_key).unwrap();
    let public_key = public_key.public_key;

    ProxyConfig {
        user_service_host,
        user_service_port,
        user_service_url,
        public_key,
        content_service_host,
        content_service_url,
        content_service_port,
    }
}
