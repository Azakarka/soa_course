use std::str::FromStr;

use uuid::Uuid;

use crate::{database::schema::{WallPostPostgres}, error::ContentServiceError};

use super::{CreatePostRequest, WallPostContent, WallPostWithMeta};


pub fn fill_wallpost_postgres_with_wallpost_content(post_content: &mut WallPostPostgres, post: WallPostContent) -> Result<(), ContentServiceError> {
    post_content.title = post.title;
    post_content.description = post.description;
    post_content.is_private = post.is_private.unwrap_or(false);
    post_content.tags = post.tags;
    Ok(())
}

pub fn fill_wallpost_with_postgres_wallpost(post: &mut WallPostContent, post_postgres: &WallPostPostgres) -> Result<(), ContentServiceError> {
    post.title = post_postgres.title.clone();
    post.description = post_postgres.description.clone();
    post.is_private = Some(post_postgres.is_private);
    post.tags = post_postgres.tags.clone();
    Ok(())
}

pub fn fill_wallpost_with_meta_with_postgres_wallpost(post_meta: &mut WallPostWithMeta, post: WallPostPostgres) -> Result<(), ContentServiceError> {
    post_meta.content = Some(WallPostContent::default());
    fill_wallpost_with_postgres_wallpost(&mut post_meta.content.as_mut().unwrap(), &post).unwrap();
    post_meta.creator_id = post.creator_id.to_string();
    post_meta.post_id = post.post_id.to_string();
    post_meta.created_at = post.created_at.unwrap().timestamp();
    post_meta.updated_at = post.updated_at.unwrap().timestamp();
    Ok(())
}
