mod api;
mod models;
mod repository;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
// use api::user_api::{create_user, delete_user, get_user, update_user};
use repository::mongodb_repos::MongoRepo;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new().app_data(db_data.clone()).service(index)
        // .service(create_user)
        // .service(get_user)
        // .service(update_user)
        // .service(delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
