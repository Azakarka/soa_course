use deadpool_postgres::Client;
use deadpool_postgres::Pool;

use crate::error::ContentServiceError;

async fn prepare_post(client: &Client) {
    let stmt_ = include_str!("../sql_queries/create_posts_table.pgsql");
    let stmt = client.prepare(&stmt_).await.unwrap();
    client.query(&stmt, &[]).await.unwrap();
}
async fn prepare_likes(client: &Client) {
    let stmt_ = include_str!("../sql_queries/create_likes_table.pgsql");
    let stmt = client.prepare(&stmt_).await.unwrap();
    client.query(&stmt, &[]).await.unwrap();
}

async fn prepare_comments(client: &Client) {
    let stmt_ = include_str!("../sql_queries/create_comments_table.pgsql");
    let stmt = client.prepare(&stmt_).await.unwrap();
    client.query(&stmt, &[]).await.unwrap();
}

pub async fn warmup(db_pool: &Pool) -> Result<(), ContentServiceError> {
    let client: Client = db_pool.get().await.unwrap();
    prepare_post(&client).await;
    prepare_likes(&client).await;
    prepare_comments(&client).await;
    Ok(())
}
