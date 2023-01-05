use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, repository::mongodb_repos::MongoRepo};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl MongoRepo {
    pub async fn user_exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        let filter = doc! {"email": email};
        let user = self.user_col.find_one(filter, None).await?;

        if user.is_none() {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let filter = doc! {"email": email};
        let user = self.user_col.find_one(filter, None).await?;

        Ok(user)
    }

    pub async fn create_user(&self, dto: SignUpDTO) -> Result<InsertOneResult, AppError> {
        let user = User {
            id: None,
            name: dto.name,
            email: dto.email,
            password: dto.password,
        };
        let result = self.user_col.insert_one(user, None).await?;

        Ok(result)
    }
}
