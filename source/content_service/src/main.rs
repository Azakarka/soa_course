use content_service::{content_service_server::ContentServiceServer, server::ContentServiceInfo};
use tokio_postgres::NoTls;
mod config;
mod content_service;
pub mod error;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config();

    let pool = config.postgres_config.create_pool(None, NoTls).unwrap();
    database::warmup::warmup(&pool).await.unwrap();

    let server = ContentServiceServer::new(ContentServiceInfo { pool });
    println!("Server running at http://{}", "0.0.0.0:8083");
    println!("Config: {:?}", config);
    tonic::transport::Server::builder()
        .add_service(server)
        .serve("0.0.0.0:8083".parse()?)
        .await?;
    Ok(())
}
