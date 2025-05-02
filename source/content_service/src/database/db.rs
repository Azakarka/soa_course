use crate::error::ContentServiceError;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use super::schema::{CommentPostgres, LikePostgres, WallPostPostgres};

pub async fn create_post(
    client: &Client,
    wallpost: WallPostPostgres,
) -> Result<WallPostPostgres, ContentServiceError> {
    let _stmt = include_str!("../sql_queries/create_post.pgsql");
    let _stmt = _stmt.replace("$table_fields", &WallPostPostgres::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&wallpost));

    let wallpost = client
        .query(
            &stmt,
            &[
                &wallpost.title,
                &wallpost.description,
                &wallpost.creator_id,
                &wallpost.is_private,
                &wallpost.tags,
            ],
        )
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| WallPostPostgres::from_row_ref(row).unwrap())
        .next();
    assert!(wallpost.is_some());
    Ok(wallpost.unwrap())
}

pub async fn delete_post(
    client: &Client,
    wallpost: WallPostPostgres,
) -> Result<WallPostPostgres, ContentServiceError> {
    let stmt = include_str!("../sql_queries/delete_post.pgsql");
    let stmt = stmt.replace("$table_fields", &WallPostPostgres::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let wallpost = client
        .query(&stmt, &[&wallpost.post_id])
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| WallPostPostgres::from_row_ref(row).unwrap())
        .next();
    assert!(wallpost.is_some());
    Ok(wallpost.unwrap())
}

pub async fn update_post(
    client: &Client,
    wallpost: WallPostPostgres,
) -> Result<WallPostPostgres, ContentServiceError> {
    let _stmt = include_str!("../sql_queries/update_post.pgsql");
    let _stmt = _stmt.replace("$table_fields", &WallPostPostgres::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&wallpost));

    let wallpost = client
        .query(
            &stmt,
            &[
                &wallpost.post_id,
                &wallpost.title,
                &wallpost.description,
                &wallpost.is_private,
                &wallpost.tags,
            ],
        )
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| WallPostPostgres::from_row_ref(row).unwrap())
        .next();
    assert!(wallpost.is_some());
    Ok(wallpost.unwrap())
}

pub async fn get_post(
    client: &Client,
    wallpost: WallPostPostgres,
) -> Result<WallPostPostgres, ContentServiceError> {
    let _stmt = include_str!("../sql_queries/get_post_by_post_id.pgsql");
    let _stmt = _stmt.replace("$table_fields", &WallPostPostgres::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&wallpost));

    let wallpost = client
        .query(&stmt, &[&wallpost.post_id])
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| WallPostPostgres::from_row_ref(row).unwrap())
        .next();
    assert!(wallpost.is_some());
    Ok(wallpost.unwrap())
}

pub async fn get_posts(
    client: &Client,
    wallpost: WallPostPostgres,
    page: i64,
    limit: i64,
    user_id: Uuid,
) -> Result<Vec<WallPostPostgres>, ContentServiceError> {
    let _stmt = include_str!("../sql_queries/get_posts_paged.pgsql");
    let _stmt = _stmt.replace("$table_fields", &WallPostPostgres::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&wallpost));
    let results = client
        .query(&stmt, &[&wallpost.creator_id, &limit, &page, &user_id])
        .await
        .unwrap()
        .iter()
        .map(|row| WallPostPostgres::from_row_ref(row).unwrap())
        .collect();
    Ok(results)
}

pub async fn create_like(
    client: &Client,
    like: LikePostgres,
) -> Result<LikePostgres, ContentServiceError> {
    let _stmt = include_str!("../sql_queries/create_like.pgsql");
    let _stmt = _stmt.replace("$table_fields", &LikePostgres::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&like));

    let like = client
        .query(&stmt, &[&like.post_id, &like.user_id])
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| LikePostgres::from_row_ref(row).unwrap())
        .next();
    assert!(like.is_some());
    Ok(like.unwrap())
}

pub async fn create_comment(
    client: &Client,
    comment: CommentPostgres,
) -> Result<CommentPostgres, ContentServiceError> {
    let _stmt = include_str!("../sql_queries/create_comment.pgsql");
    let _stmt = _stmt.replace("$table_fields", &CommentPostgres::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&comment));

    let comment = client
        .query(&stmt, &[&comment.post_id, &comment.user_id, &comment.text])
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| CommentPostgres::from_row_ref(row).unwrap())
        .next();
    assert!(comment.is_some());
    Ok(comment.unwrap())
}

pub async fn get_comments(
    client: &Client,
    post_id: Uuid,
    page: i64,
    limit: i64,
) -> Result<Vec<CommentPostgres>, ContentServiceError> {
    let _stmt = include_str!("../sql_queries/get_comments_paged.pgsql");
    let _stmt = _stmt.replace("$table_fields", &CommentPostgres::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    let results = client
        .query(&stmt, &[&post_id, &limit, &page])
        .await
        .unwrap()
        .iter()
        .map(|row| CommentPostgres::from_row_ref(row).unwrap())
        .collect();
    Ok(results)
}
