use actix_web::http::Error;
use deadpool_postgres::Client;
use deadpool_postgres::Pool;

pub async fn warmup(db_pool: &Pool) -> Result<(), Error> {
    let client: Client = db_pool.get().await.unwrap();
    let stmt_ = include_str!("../sql_queries/create_tables.pgsql");
    let stmt = client.prepare(&stmt_).await.unwrap();

    client.query(&stmt, &[]).await.unwrap();

    Ok(())
}
