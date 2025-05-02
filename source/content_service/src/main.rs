use content_service::{content_service_server::ContentServiceServer, server::ContentServiceInfo};
use tokio_postgres::NoTls;
mod config;
mod content_service;
mod database;
pub mod error;
mod kafka;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config();

    let pool = config.postgres_config.create_pool(None, NoTls).unwrap();
    database::warmup::warmup(&pool).await.unwrap();

    let host = format!(
        "{}:{}",
        config.server_config.host, config.server_config.port
    );

    println!("Server running at http://{}", &host);
    println!("Config: {:?}", &config);
    let server = ContentServiceServer::new(ContentServiceInfo { pool, config });
    tonic::transport::Server::builder()
        .add_service(server)
        .serve(host.parse()?)
        .await?;
    Ok(())
}
