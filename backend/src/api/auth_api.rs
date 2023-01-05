use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    auth::auth_token::{Claims, Token},
    constants,
    errors::AppError,
    models::user_model::{LoginDTO, SignUpDTO},
    repository::mongodb_repos::MongoRepo,
};

#[post("/signup")]
pub async fn signup(db: Data<MongoRepo>, data: Json<SignUpDTO>) -> Result<HttpResponse, AppError> {
    let user_exists = db.user_exists_by_email(&data.email).await?;
    if user_exists {
        return Err(AppError::user_error(constants::MESSAGE_USER_ALREADY_EXISTS));
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

#[post("/login")]
pub async fn login(db: Data<MongoRepo>, data: Json<LoginDTO>) -> Result<HttpResponse, AppError> {
    let user = match db.find_user_by_email(&data.email).await? {
        Some(v) => v,
        None => {
            return Err(AppError::user_error(
                constants::MESSAGE_EMAIL_PASSWORD_INCORRECT,
            ));
        }
    };

    let is_valid_password = bcrypt::verify(&data.password, &user.password)?;
    if !is_valid_password {
        return Err(AppError::user_error(
            constants::MESSAGE_EMAIL_PASSWORD_INCORRECT,
        ));
    }

    Ok(HttpResponse::Ok().json(Token::new_bearer(Claims::generate_token(user.id)?)))
}
