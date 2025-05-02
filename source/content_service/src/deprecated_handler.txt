use core::panic;
use std::str::FromStr;

use actix_web::{
    get,
    http::header::{HeaderName, HeaderValue},
    post, web, Error, HttpResponse,
};
use crate::{cert::token::{create_token, validate_token}, config::UserServiceConfig, error::UserServiceError};
use crate::database::{
    db,
    schema::UserProfile,
};
use deadpool_postgres::{Client, Pool};
use log::debug;
use serde_json::json;
use uuid::Uuid;


#[get("/profile/{user_uuid}")]
pub async fn get_profile(
    uuid: web::Path<Uuid>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    println!("Get profile info with uuid: {:?}", uuid);
    let client: Client = db_pool.get().await.unwrap();

    let users = db::get_profile(&client, *uuid).await?;

    Ok(HttpResponse::Ok().json(users))
}

#[post("/profile/update/{user_uuid}")]
pub async fn update_profile(
    uuid: web::Path<Uuid>,
    user: web::Json<UserProfile>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    debug!("Update: {:?}", user);
    let user_info: UserProfile = user.into_inner();

    let client: Client = db_pool.get().await.unwrap();

    let profile = db::update_user(&client, user_info, *uuid).await?;

    let response = HttpResponse::Ok().json(profile);
    Ok(response)
}

#[post("/register")]
pub async fn add_profile(
    user: web::Json<UserProfile>,
    db_pool: web::Data<Pool>,
    config: web::Data<UserServiceConfig>,
) -> Result<HttpResponse, Error> {
    println!("New register!");
    let user_info: UserProfile = user.into_inner();

    let client: Client = db_pool.get().await.unwrap();

    let profile = db::add_user(&client, user_info).await?;

    let token = create_token(profile.user_uuid.unwrap().clone(), &config.get_ref()).unwrap();

    println!("Generated token: {token}");
    println!(
        "UUid: {}",
        validate_token(token.clone(), config.public_key.clone()).unwrap()
    );

    let mut response = HttpResponse::Ok().json(profile);
    response.headers_mut().append(
        HeaderName::from_str("Authorization").unwrap(),
        HeaderValue::from_str(&token).unwrap(),
    );
    Ok(response)
}

#[post("/login")]
pub async fn login_profile(
    user: web::Json<UserProfile>,
    db_pool: web::Data<Pool>,
    config: web::Data<UserServiceConfig>,
) -> Result<HttpResponse, Error> {
    debug!("Login");
    let user_info: UserProfile = user.into_inner();

    let client: Client = db_pool.get().await.unwrap();

    let profile = match db::login_profile(&client, user_info).await {
        Ok(profile) => profile,
        Err(e) => {
            match e {
                UserServiceError::WrongPassword => return Ok(HttpResponse::Forbidden().body("Invalid password")),
                UserServiceError::UserNotExists(name) => return Ok(HttpResponse::NotFound().body(format!("User {name} not found"))),
            }
        }
    };

    let token = create_token(profile.user_uuid.unwrap().clone(), &config.get_ref()).unwrap();

    debug!("token: {}", token.clone());

    let mut response = HttpResponse::Ok().json(profile);
    response.headers_mut().append(
        HeaderName::from_str("Authorization").unwrap(),
        HeaderValue::from_str(&token).unwrap(),
    );
    Ok(response)
}

#[get("/get_public_key")]
pub async fn get_public_key(config: web::Data<UserServiceConfig>) -> Result<HttpResponse, Error> {
    let resp = json!({"public_key": config.public_key});
    debug!("Sening: {}", resp);
    Ok(HttpResponse::Ok().json(resp))
}
