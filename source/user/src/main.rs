use actix_web::{
    web, App, HttpServer,
};
use tokio_postgres::NoTls;


mod cert;
mod database;
mod handler;
mod config;
pub mod error;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config();

    let pool = config.postgres_config.create_pool(None, NoTls).unwrap();
    println!("Config: {:?}", config);

    database::warmup::warmup(&pool).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(handler::get_profile)
            .service(handler::add_profile)
            .service(handler::update_profile)
            .service(handler::login_profile)
            .service(handler::get_public_key)
    })
    .bind("0.0.0.0:8082")?
    .run();
    println!("Server running at http://{}/", "0.0.0.0:8082");

    server.await
}
