use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

use crate::{models::user_model::User, repository::mongodb_repos::MongoRepo};

// #[post("/user")]
// pub async fn create_user(db: Data<MongoRepo>, data: Json<User>) -> HttpResponse {
//     let new_user = User {
//         id: None,
//         name: data.name.to_owned(),
//     };
//     match db.create_user(new_user).await {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[get("/user/{id}")]
// pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }

//     match db.get_user(&id).await {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[put("/user/{id}")]
// pub async fn update_user(
//     db: Data<MongoRepo>,
//     path: Path<String>,
//     new_user: Json<User>,
// ) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }

//     let data = User {
//         id: Some(ObjectId::parse_str(&id).unwrap()),
//         name: new_user.name.to_owned(),
//     };
//     let update_result = db.update_user(&id, data).await;
//     match update_result {
//         Ok(update) => {
//             if update.matched_count == 1 {
//                 let update_user_info = db.get_user(&id).await;
//                 match update_user_info {
//                     Ok(user) => HttpResponse::Ok().json(user),
//                     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//                 }
//             } else {
//                 HttpResponse::NotFound().body("No user found with specified ID")
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[delete("/user/{id}")]
// pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }

//     let delete_result = db.delete_user(&id).await;
//     match delete_result {
//         Ok(delete) => {
//             if delete.deleted_count == 1 {
//                 HttpResponse::Ok().json("User successfully deleted")
//             } else {
//                 HttpResponse::NotFound().json("User with specified ID not found!")
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }
