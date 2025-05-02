use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::user::config::ProxyConfig;

use crate::cert::token::validate_token;

#[get("/profile")]
pub async fn get_profile_info(req: HttpRequest, config: web::Data<ProxyConfig>) -> impl Responder {
    debug!("Get profile");
    debug!("Cookies: {:?}", req.cookies());
    let jwt = match req.cookie("jwt-token") {
        Some(token) => token.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No jwt token specified"),
    };
    let public_key = config.public_key.clone();
    let uuid = validate_token(jwt, public_key).unwrap();
    let url = format_args!("{}/profile/{}", config.user_service_url, uuid).to_string();
    let client = reqwest::Client::new();
    let res = client.get(url).send().await.unwrap();
    HttpResponse::Ok().body(res.bytes().await.unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct UpdateMessage {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub birth_date: Option<chrono::DateTime<Utc>>,
}

#[put("/profile")]
pub async fn update_profile(
    req: HttpRequest,
    update_message: web::Json<UpdateMessage>,
    config: web::Data<ProxyConfig>,
) -> impl Responder {
    debug!("New Update");
    let jwt = match req.cookie("jwt-token") {
        Some(token) => token.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No jwt token specified"),
    };
    let public_key = config.public_key.clone();
    let uuid = validate_token(jwt, public_key).unwrap();
    let url = format_args!("{}/profile/update/{}", config.user_service_url, uuid).to_string();
    let client = reqwest::Client::new();
    let res = client.post(url).json(&update_message).send().await.unwrap();
    HttpResponse::Ok().body(res.bytes().await.unwrap())
}
