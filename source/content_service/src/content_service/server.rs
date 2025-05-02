use std::str::FromStr;

use deadpool_postgres::Pool;
use tonic::Response;
use uuid::Uuid;

use crate::{
    config::Config,
    content_service::{
        content_service_server::ContentService,
        types_mapper::{self},
        Comment, CommentPostResponse, CreatePostRequest, CreatePostResponse, DeletePostRequest,
        DeletePostResponse, GetPostRequest, GetPostResponse, GetPostsRequest, GetPostsResponse,
        UpdatePostRequest, UpdatePostResponse, WallPostWithMeta,
    },
    database::{
        self,
        schema::{CommentPostgres, LikePostgres, WallPostPostgres},
    },
};

use super::{
    CommentPostRequest, GetCommentsRequest, GetCommentsResponse, LikePostRequest, LikePostResponse,
};

#[derive(Debug)]
pub struct ContentServiceInfo {
    pub pool: Pool,
    pub config: Config,
}

#[tonic::async_trait]
impl ContentService for ContentServiceInfo {
    async fn create_post(
        &self,
        request: tonic::Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, tonic::Status> {
        println!("Create post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        types_mapper::fill_wallpost_postgres_with_wallpost_content(
            &mut wallpost,
            request.post.unwrap(),
        )
        .unwrap();
        wallpost.creator_id = Uuid::from_str(request.creator_id.as_str()).unwrap();

        let res = database::db::create_post(&self.pool.get().await.unwrap(), wallpost)
            .await
            .unwrap();
        println!("Create post response: {:?}", res);
        let mut wallpost_with_meta = WallPostWithMeta::default();
        types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(&mut wallpost_with_meta, res)
            .unwrap();
        Ok(Response::new(CreatePostResponse {
            post: Some(wallpost_with_meta),
        }))
    }

    async fn delete_post(
        &self,
        request: tonic::Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, tonic::Status> {
        println!("Delete post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        wallpost.post_id = Uuid::from_str(request.post_id.as_str()).unwrap();

        let res = database::db::delete_post(&self.pool.get().await.unwrap(), wallpost)
            .await
            .unwrap();
        println!("Delete post response: {:?}", res);
        Ok(Response::new(DeletePostResponse {
            id: request.post_id,
        }))
    }

    async fn update_post(
        &self,
        request: tonic::Request<UpdatePostRequest>,
    ) -> Result<Response<UpdatePostResponse>, tonic::Status> {
        println!("Update post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        types_mapper::fill_wallpost_postgres_with_wallpost_content(
            &mut wallpost,
            request.post.unwrap(),
        )
        .unwrap();
        wallpost.post_id = Uuid::from_str(request.post_id.as_str()).unwrap();

        let res = database::db::update_post(&self.pool.get().await.unwrap(), wallpost)
            .await
            .unwrap();
        println!("Update post response: {:?}", res);
        let mut wallpost_with_meta = WallPostWithMeta::default();
        types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(&mut wallpost_with_meta, res)
            .unwrap();
        Ok(Response::new(UpdatePostResponse {
            post: Some(wallpost_with_meta),
        }))
    }

    async fn get_post(
        &self,
        request: tonic::Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, tonic::Status> {
        println!("Get post");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        wallpost.post_id = Uuid::from_str(request.post_id.as_str()).unwrap();
        wallpost.creator_id = Uuid::from_str(&request.user_id).unwrap();

        crate::kafka::producers::produce_post_view_event(
            request.post_id.clone(),
            request.user_id.clone(),
            &self.config,
        )
        .unwrap();

        let res: WallPostPostgres =
            database::db::get_post(&self.pool.get().await.unwrap(), wallpost)
                .await
                .unwrap();
        println!("Get post response: {:?}", res);
        if res.is_private && res.creator_id != Uuid::from_str(&request.user_id).unwrap() {
            return Err(tonic::Status::new(
                tonic::Code::PermissionDenied,
                "Permission denied",
            ));
        }
        let mut wallpost_with_meta = WallPostWithMeta::default();
        types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(&mut wallpost_with_meta, res)
            .unwrap();
        Ok(Response::new(GetPostResponse {
            post: Some(wallpost_with_meta),
        }))
    }

    async fn get_posts(
        &self,
        request: tonic::Request<GetPostsRequest>,
    ) -> Result<Response<GetPostsResponse>, tonic::Status> {
        println!("Get posts");
        let request = request.into_inner();
        let mut wallpost = WallPostPostgres::default();
        wallpost.creator_id = Uuid::from_str(&request.creator_id).unwrap();

        let res_vec: Vec<WallPostPostgres> = database::db::get_posts(
            &self.pool.get().await.unwrap(),
            wallpost,
            request.page as i64,
            request.limit as i64,
            Uuid::from_str(&request.user_id).unwrap(),
        )
        .await
        .unwrap();
        println!("Get posts response: {:?}", res_vec);
        let mut wallposts_with_meta = Vec::new();
        for res in res_vec.into_iter() {
            let mut wallpost_with_meta = WallPostWithMeta::default();
            types_mapper::fill_wallpost_with_meta_with_postgres_wallpost(
                &mut wallpost_with_meta,
                res,
            )
            .unwrap();
            wallposts_with_meta.push(wallpost_with_meta);
        }
        Ok(Response::new(GetPostsResponse {
            posts: wallposts_with_meta,
        }))
    }

    async fn like_post(
        &self,
        request: tonic::Request<LikePostRequest>,
    ) -> Result<Response<LikePostResponse>, tonic::Status> {
        println!("Like post");
        let request = request.into_inner();
        let mut like = LikePostgres::default();
        types_mapper::fill_like_postgres_with_like_request(&mut like, &request).unwrap();

        crate::kafka::producers::produce_post_like_event(
            request.post_id.clone(),
            request.user_id.clone(),
            &self.config,
        )
        .unwrap();

        let res = database::db::create_like(&self.pool.get().await.unwrap(), like)
            .await
            .unwrap();
        println!("Create like response: {:?}", res);
        let mut like_response = LikePostResponse::default();
        types_mapper::fill_like_response_with_like_postgres(&mut like_response, res).unwrap();
        Ok(Response::new(like_response))
    }

    async fn comment_post(
        &self,
        request: tonic::Request<CommentPostRequest>,
    ) -> Result<Response<CommentPostResponse>, tonic::Status> {
        println!("Comment post");
        let request = request.into_inner();
        let mut comment = CommentPostgres::default();
        types_mapper::fill_comment_postgres_with_comment_request(&mut comment, &request).unwrap();

        crate::kafka::producers::produce_post_comment_event(
            request.post_id.clone(),
            request.user_id.clone(),
            &self.config,
        )
        .unwrap();

        let res = database::db::create_comment(&self.pool.get().await.unwrap(), comment)
            .await
            .unwrap();
        println!("Create like response: {:?}", res);
        let mut comment_response = CommentPostResponse::default();
        types_mapper::fill_comment_response_with_comment_postgres(&mut comment_response, res)
            .unwrap();
        Ok(Response::new(comment_response))
    }

    async fn get_comments(
        &self,
        request: tonic::Request<GetCommentsRequest>,
    ) -> Result<Response<GetCommentsResponse>, tonic::Status> {
        println!("Get comments for post");
        let request = request.into_inner();
        let res = database::db::get_comments(
            &self.pool.get().await.unwrap(),
            Uuid::from_str(&request.post_id).unwrap(),
            request.page as i64,
            request.limit as i64,
        )
        .await
        .unwrap();
        println!("Create like response: {:?}", res);
        let mut response = GetCommentsResponse::default();
        for comment in res.iter() {
            let mut comment_res = Comment::default();
            types_mapper::fill_comment_with_comment_postgres(&mut comment_res, comment).unwrap();
            response.comments.push(comment_res);
        }
        Ok(Response::new(response))
    }
}
