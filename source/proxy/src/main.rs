use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, info, trace};
use user::{auth, config::get_config, profile};

mod cert;
mod user;

#[get("hello")]
pub async fn hello() -> impl Responder {
    trace!("Hello!");
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    debug!("WtF");
    let config = get_config().await;
    info!("Config: {:?}", config);

    info!("Starting proxy server at http://0.0.0.0:8081");
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(config.clone())).service(
            web::scope("/auth")
                .service(auth::register)
                .service(auth::login)
                .service(profile::get_profile_info)
                .service(profile::update_profile)
                .service(hello),
        )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
