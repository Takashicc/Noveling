mod api;
mod auth;
mod config;
mod constants;
mod errors;
mod models;
mod repository;

use actix_cors::Cors;
use actix_web::{
    http::{header, Method},
    web::Data,
    App, HttpServer,
};
use repository::mongodb_repos::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
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
            .configure(config::api_config::config_services)
    })
    .bind((constants::CONFIG.host.as_ref(), constants::CONFIG.port))?
    .run()
    .await
}
