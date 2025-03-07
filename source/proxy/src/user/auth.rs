use std::str::FromStr;

use actix_web::{cookie::CookieBuilder, post, web, HttpResponse, Responder};
use log::debug;
use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};

use crate::user::config::ProxyConfig;

#[derive(Serialize, Deserialize)]
struct RegisterMessage {
    username: String,
    password: String,
    email: String,
}

#[post("/register")]
pub async fn register(
    register_message: web::Json<RegisterMessage>,
    config: web::Data<ProxyConfig>,
) -> impl Responder {
    debug!("New register!");
    let client = reqwest::Client::new();
    let url = format_args!("{}/register", config.user_service_url).to_string();
    let url = reqwest::Url::from_str(&url).unwrap();
    debug!("Sending to {}", &url);
    let res = client
        .post(url.clone())
        .json(&register_message)
        .send()
        .await
        .unwrap();
    debug!("Code: {}", res.status());
    match res.status() {
        StatusCode::OK => {
            let token = res
                .headers()
                .get("Authorization")
                .unwrap()
                .to_str()
                .unwrap();
            let token_cookie = CookieBuilder::new("jwt-token", token).finish();
            HttpResponse::Ok()
                .cookie(token_cookie)
                .body(res.text().await.unwrap())
        }
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().body(res.text().await.unwrap()),
        _ => HttpResponse::InternalServerError().body(res.text().await.unwrap()),
    }
}

#[derive(Serialize, Deserialize)]
struct LoginMessage {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    login_message: web::Json<LoginMessage>,
    config: web::Data<ProxyConfig>,
) -> impl Responder {
    debug!("Login");
    let client = reqwest::Client::new();
    let url = format_args!("{}/login", config.user_service_url).to_string();
    let res = client.post(url).json(&login_message).send().await.unwrap();
    match res.status() {
        StatusCode::OK => {
            let token = res
                .headers()
                .get("Authorization")
                .unwrap()
                .to_str()
                .unwrap();
            let token_cookie = CookieBuilder::new("jwt-token", token).finish();
            HttpResponse::Ok()
                .cookie(token_cookie)
                .body(res.text().await.unwrap())
        }
        StatusCode::FORBIDDEN => HttpResponse::Forbidden().body(res.text().await.unwrap()),
        StatusCode::NOT_FOUND => HttpResponse::NotFound().body(res.text().await.unwrap()),
        _ => HttpResponse::InternalServerError().body(res.text().await.unwrap()),
    }
}
