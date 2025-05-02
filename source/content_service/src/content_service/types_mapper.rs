use std::str::FromStr;

use uuid::Uuid;

use crate::{
    database::schema::{CommentPostgres, LikePostgres, WallPostPostgres},
    error::ContentServiceError,
};

use super::{
    Comment, CommentPostRequest, CommentPostResponse, LikePost, LikePostRequest, LikePostResponse,
    WallPostContent, WallPostWithMeta,
};

pub fn fill_wallpost_postgres_with_wallpost_content(
    post_content: &mut WallPostPostgres,
    post: WallPostContent,
) -> Result<(), ContentServiceError> {
    post_content.title = post.title;
    post_content.description = post.description;
    post_content.is_private = post.is_private.unwrap_or(false);
    post_content.tags = post.tags;
    Ok(())
}

pub fn fill_wallpost_with_postgres_wallpost(
    post: &mut WallPostContent,
    post_postgres: &WallPostPostgres,
) -> Result<(), ContentServiceError> {
    post.title = post_postgres.title.clone();
    post.description = post_postgres.description.clone();
    post.is_private = Some(post_postgres.is_private);
    post.tags = post_postgres.tags.clone();
    Ok(())
}

pub fn fill_wallpost_with_meta_with_postgres_wallpost(
    post_meta: &mut WallPostWithMeta,
    post: WallPostPostgres,
) -> Result<(), ContentServiceError> {
    post_meta.content = Some(WallPostContent::default());
    fill_wallpost_with_postgres_wallpost(&mut post_meta.content.as_mut().unwrap(), &post).unwrap();
    post_meta.creator_id = post.creator_id.to_string();
    post_meta.post_id = post.post_id.to_string();
    post_meta.created_at = post.created_at.unwrap().timestamp();
    post_meta.updated_at = post.updated_at.unwrap().timestamp();
    Ok(())
}

pub fn fill_like_postgres_with_like_request(
    like: &mut LikePostgres,
    like_request: &LikePostRequest,
) -> Result<(), ContentServiceError> {
    like.post_id = Uuid::from_str(&like_request.post_id).unwrap();
    like.user_id = Uuid::from_str(&like_request.user_id).unwrap();
    Ok(())
}

pub fn fill_like_response_with_like_postgres(
    like_response: &mut LikePostResponse,
    like: LikePostgres,
) -> Result<(), ContentServiceError> {
    let likepost = LikePost {
        post_id: like.post_id.to_string(),
        user_id: like.user_id.to_string(),
        created_at: like.created_at.unwrap().timestamp(),
    };
    like_response.like = Some(likepost);
    Ok(())
}

pub fn fill_comment_postgres_with_comment_request(
    comment: &mut CommentPostgres,
    comment_request: &CommentPostRequest,
) -> Result<(), ContentServiceError> {
    comment.post_id = Uuid::from_str(&comment_request.post_id).unwrap();
    comment.user_id = Uuid::from_str(&comment_request.user_id).unwrap();
    comment.text = comment_request.text.clone();
    Ok(())
}

pub fn fill_comment_response_with_comment_postgres(
    comment_response: &mut CommentPostResponse,
    comment_postgres: CommentPostgres,
) -> Result<(), ContentServiceError> {
    let comment = Comment {
        comment_id: comment_postgres.comment_id.to_string(),
        post_id: comment_postgres.post_id.to_string(),
        user_id: comment_postgres.user_id.to_string(),
        text: comment_postgres.text.clone(),
        created_at: comment_postgres.created_at.unwrap().timestamp(),
    };
    comment_response.comment = Some(comment);
    Ok(())
}
pub fn fill_comment_with_comment_postgres(
    comment: &mut Comment,
    comment_postgres: &CommentPostgres,
) -> Result<(), ContentServiceError> {
    comment.comment_id = comment_postgres.comment_id.to_string();
    comment.post_id = comment_postgres.post_id.to_string();
    comment.user_id = comment_postgres.user_id.to_string();
    comment.text = comment_postgres.text.clone();
    comment.created_at = comment_postgres.created_at.unwrap().timestamp();
    Ok(())
}
