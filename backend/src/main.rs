mod api;
mod config;
mod constants;
mod errors;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use repository::mongodb_repos::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(config::api_config::config_services)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
