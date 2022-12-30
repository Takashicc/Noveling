use crate::models::user_model::User;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::env;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();

        let uri = env::var("MONGO_URI").expect("Failed to load MONGO_URI");
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("noveling");
        db.run_command(doc! {"ping": 1}, None)
            .await
            .expect("Unable to connect to database");
        println!("Successfully connected to database");

        let col: Collection<User> = db.collection("User");

        MongoRepo { col }
    }

    // pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
    //     let doc = User {
    //         id: None,
    //         name: new_user.name,
    //     };
    //     let user = self
    //         .col
    //         .insert_one(doc, None)
    //         .await
    //         .expect("Error creating user");

    //     Ok(user)
    // }

    // pub async fn get_user(&self, id: &String) -> Result<User, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let user = self
    //         .col
    //         .find_one(filter, None)
    //         .await
    //         .expect("Error getting user's detail");

    //     Ok(user.unwrap())
    // }

    // pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {
    //         "_id": obj_id
    //     };
    //     let new_doc = doc! {
    //         "$set": {
    //             "id": new_user.id,
    //             "name": new_user.name,
    //         }
    //     };
    //     let update_doc = self
    //         .col
    //         .update_one(filter, new_doc, None)
    //         .await
    //         .expect("Error updating user");

    //     Ok(update_doc)
    // }

    // pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {
    //         "_id": obj_id
    //     };
    //     let user_detail = self
    //         .col
    //         .delete_one(filter, None)
    //         .await
    //         .expect("Error deleting user");

    //     Ok(user_detail)
    // }
}