mod api;
mod auth;
mod config;
mod constants;
mod errors;
mod models;
mod repository;

use std::env;

use actix_cors::Cors;
use actix_web::{
    http::{header, Method},
    middleware::Logger,
    web::Data,
    App, HttpServer,
};
use repository::mongodb_repos::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var(
        "RUST_LOG",
        format!("actix_web={}", constants::CONFIG.log_level),
    );
    env_logger::init();

    let db = MongoRepo::init(&constants::CONFIG.mongo_uri).await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .configure(config::api_config::config_services)
    })
    .bind((constants::CONFIG.host.as_ref(), constants::CONFIG.port))?
    .run()
    .await
}
