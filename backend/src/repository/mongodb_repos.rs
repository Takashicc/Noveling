use crate::{constants, models::user_model::User};
use mongodb::{bson::doc, Client, Collection};

pub struct MongoRepo {
    pub user_col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let uri = &constants::CONFIG.mongo_uri;
        let client = Client::with_uri_str(uri)
            .await
            .expect("Failed to parse Mongo URI");
        let db = client.database("noveling");
        db.run_command(doc! {"ping": 1}, None)
            .await
            .expect("Unable to connect to database");
        println!("Successfully connected to database");

        let user_col: Collection<User> = db.collection("User");

        MongoRepo { user_col }
    }

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
