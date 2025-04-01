use std::str::FromStr;

use deadpool_postgres::Pool;
use tonic::Response;
use uuid::Uuid;

use crate::{content_service::{content_service_server::ContentService, types_mapper::{self}, CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest, GetPostResponse, GetPostsRequest, GetPostsResponse, UpdatePostRequest, UpdatePostResponse, WallPostWithMeta}, database::{self, schema::WallPostPostgres}};


#[derive(Debug)]
pub struct ContentServiceInfo {
    pub pool: Pool
}

#[tonic::async_trait]
impl ContentService for ContentServiceInfo {

    async fn create_post(&self, request: tonic::Request<CreatePostRequest>) -> Result<Response<CreatePostResponse>, tonic::Status> {
        println!("Create post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        types_mapper::fill_wallpost_postgres_with_wallpost_content(&mut wallpost, request.post.unwrap()).unwrap();
        wallpost.creator_id = Uuid::from_str(request.creator_id.as_str()).unwrap();

        let res = database::db::create_post(&self.pool.get().await.unwrap(), wallpost).await.unwrap();
        println!("Create post response: {:?}", res);
        let mut wallpost_with_meta = WallPostWithMeta::default();
        types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(&mut wallpost_with_meta, res).unwrap();
        Ok(
            Response::new(CreatePostResponse {
               post: Some(wallpost_with_meta)
            })
        )
    }

    async fn delete_post(&self, request: tonic::Request<DeletePostRequest>) -> Result<Response<DeletePostResponse>, tonic::Status> {
        println!("Delete post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        wallpost.post_id = Uuid::from_str(request.post_id.as_str()).unwrap();

        let res = database::db::delete_post(&self.pool.get().await.unwrap(), wallpost).await.unwrap();
        println!("Delete post response: {:?}", res);
        Ok (
            Response::new(DeletePostResponse {
                id: request.post_id
            })
        )
    }

    async fn update_post(&self, request: tonic::Request<UpdatePostRequest>) -> Result<Response<UpdatePostResponse>, tonic::Status> {
        println!("Update post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        types_mapper::fill_wallpost_postgres_with_wallpost_content(&mut wallpost, request.post.unwrap()).unwrap();
        wallpost.post_id = Uuid::from_str(request.post_id.as_str()).unwrap();

        let res = database::db::update_post(&self.pool.get().await.unwrap(), wallpost).await.unwrap();
        println!("Update post response: {:?}", res);
        let mut wallpost_with_meta = WallPostWithMeta::default();
        types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(&mut wallpost_with_meta, res).unwrap();
        Ok (
            Response::new(UpdatePostResponse {
                post: Some(wallpost_with_meta)
            })
        )
    }

    async fn get_post(&self, request: tonic::Request<GetPostRequest>) -> Result<Response<GetPostResponse>, tonic::Status> {
        println!("Get post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        wallpost.post_id = Uuid::from_str(request.post_id.as_str()).unwrap();
        wallpost.creator_id = Uuid::from_str(&request.user_id).unwrap();

        let res: WallPostPostgres = database::db::get_post(&self.pool.get().await.unwrap(), wallpost).await.unwrap();
        println!("Get post response: {:?}", res);
        if res.is_private && res.creator_id != Uuid::from_str(&request.user_id).unwrap() {
            return Err(tonic::Status::new(tonic::Code::PermissionDenied, "Permission denied"));
        }
        let mut wallpost_with_meta = WallPostWithMeta::default();
        types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(&mut wallpost_with_meta, res).unwrap();
        Ok (
            Response::new(GetPostResponse {
                post: Some(wallpost_with_meta)
            })
        )
    }

    async fn get_posts(&self, request: tonic::Request<GetPostsRequest>) -> Result<Response<GetPostsResponse>, tonic::Status> {
        println!("Get posts");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        wallpost.creator_id = Uuid::from_str(&request.creator_id).unwrap();

        let res_vec: Vec<WallPostPostgres> = database::db::get_posts(&self.pool.get().await.unwrap(), wallpost, request.page as i64, request.limit as i64, Uuid::from_str(&request.user_id).unwrap()).await.unwrap();
        println!("Get post response: {:?}", res_vec);
        let mut wallposts_with_meta = Vec::new();
        for res in res_vec.into_iter() {
            let mut wallpost_with_meta = WallPostWithMeta::default();
            types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(&mut wallpost_with_meta, res).unwrap();
            wallposts_with_meta.push(wallpost_with_meta);
        }
        Ok (
            Response::new(GetPostsResponse {
                posts: wallposts_with_meta
            })
        )
    }


}
