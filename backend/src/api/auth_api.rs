use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    errors::{AppError, AppErrorType},
    models::user_model::SignUpDTO,
    repository::mongodb_repos::MongoRepo,
};

#[post("/signup")]
pub async fn signup(db: Data<MongoRepo>, data: Json<SignUpDTO>) -> Result<HttpResponse, AppError> {
    let user_exists = db.user_exists_by_email(&data.email).await?;
    if user_exists {
        return Err(AppError {
            error_type: AppErrorType::AlreadyExists,
            status_code: None,
            message: Some("User already exists".to_string()),
        });
    }

    let hashed_password = bcrypt::hash(&data.password, bcrypt::DEFAULT_COST)?;
    let dto = SignUpDTO {
        name: data.name.to_owned(),
        email: data.email.to_owned(),
        password: hashed_password,
    };
    let result = db.create_user(dto).await?;
    Ok(HttpResponse::Ok().json(result))
}
