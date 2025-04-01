use std::str::FromStr;

use actix_web::{get, post, web::{self, post}, HttpRequest, HttpResponse, Responder};
use log::debug;
use serde::Deserialize;
use uuid::Uuid;

use crate::{cert::token::validate_token, content_service::{CreatePostRequest, DeletePostRequest, GetPostRequest, GetPostsRequest, UpdatePostRequest, WallPostContent}, error::ProxyError, user::config::ProxyConfig};

use super::content_service_client::ContentServiceClient;




async fn create_grpc_client(config: &ProxyConfig) -> ContentServiceClient<tonic::transport::Channel> {
    let host = format!("http://{}:{}", config.content_service_host, config.content_service_port);
    debug!("host for grpc client: {}", host.as_str());
    let channel = tonic::transport::Channel::builder(tonic::transport::Uri::from_str(host.as_str()).unwrap())
        .connect()
        .await
        .expect("Can't create a channel");
    ContentServiceClient::new(channel)
}

pub async fn get_and_vaidate_uuid_from_request(req: HttpRequest, config: &ProxyConfig) -> Result<Uuid, ProxyError> {
    let jwt = match req.cookie("jwt-token") {
        Some(token) => token.value().to_string(),
        None => return Err(ProxyError::NoJwtSpecified)
    };
    let public_key = config.public_key.clone();
    let uuid = validate_token(jwt, public_key)?;
    Ok(uuid)
}


#[post("/create")]
pub async fn create_post(
    req: HttpRequest,
    wall_post_content: web::Json<WallPostContent>,
    config: web::Data<ProxyConfig>,
) -> impl Responder {
    debug!("New CreatePost!");
    let user_id= match get_and_vaidate_uuid_from_request(req, config.get_ref()).await {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };
    let mut client = create_grpc_client(&config).await;
    let create_message = CreatePostRequest {
        post: Some(wall_post_content.into_inner()),
        creator_id: user_id.to_string(),
    };
    debug!("Create post message: {:?}", create_message);
    let res = client.create_post(create_message).await;
    debug!("Create post response: {:?}", res);
    match res {
        Ok(response) => {
            let response = response.into_inner();
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().json(json)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/delete/{id}")]
pub async fn delete_post(
    req: HttpRequest,
    id: web::Path<Uuid>,
    config: web::Data<ProxyConfig>
) -> impl Responder {
    debug!("Delete post!!!!");
    let user_id= match get_and_vaidate_uuid_from_request(req, config.get_ref()).await {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };
    let mut client = create_grpc_client(&config).await;
    let delete_message = DeletePostRequest {
        post_id: id.into_inner().to_string(),
        user_id: user_id.to_string(),
    };
    let res = client.delete_post(delete_message).await;
    match res {
        Ok(response) => {
            let response = response.into_inner();
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().json(json)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}


#[post("/update/{post_id}")]
pub async fn update_post(
    req: HttpRequest,
    post_message: web::Json<WallPostContent>,
    post_id: web::Path<Uuid>,
    config: web::Data<ProxyConfig>,
) -> impl Responder {
    debug!("Update post");
    let user_id= match get_and_vaidate_uuid_from_request(req, config.get_ref()).await {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };
    let mut client = create_grpc_client(&config).await;
    let update_message = UpdatePostRequest {
        post: Some(post_message.into_inner()),
        user_id: user_id.to_string(),
        post_id: post_id.into_inner().to_string(),
    };
    let res = client.update_post(update_message).await;
    match res {
        Ok(response) => {
            let response = response.into_inner();
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().json(json)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get/{id}")]
pub async fn get_post(
    req: HttpRequest,
    id: web::Path<Uuid>,
    config: web::Data<ProxyConfig>,
) -> impl Responder {
    debug!("Get post");
    let user_id= match get_and_vaidate_uuid_from_request(req, config.get_ref()).await {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };
    let mut client = create_grpc_client(&config).await;
    let get_message = GetPostRequest {
        post_id: id.into_inner().to_string(),
        user_id: user_id.to_string(),
    };
    let res = client.get_post(get_message).await;
    match res {
        Ok(response) => {
            let response = response.into_inner();
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().json(json)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[derive(Deserialize)]
struct MultipleGetParams {
    page: i32,
    limit: i32
}

#[get("/gets")]
pub async fn get_posts(
    req: HttpRequest,
    params: web::Query<MultipleGetParams>,
    config: web::Data<ProxyConfig>,
) -> impl Responder {
    debug!("Get Posts");
    let user_id= match get_and_vaidate_uuid_from_request(req, config.get_ref()).await {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };
    let mut client = create_grpc_client(&config).await;
    let posts_message = GetPostsRequest {
        creator_id: user_id.to_string(),
        page: params.page,
        limit: params.limit,
        user_id: user_id.to_string(),
    };
    let res = client.get_posts(posts_message).await;
    match res {
        Ok(response) => {
            let response = response.into_inner();
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().json(json)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
