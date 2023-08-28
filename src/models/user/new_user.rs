extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};
use uuid::Uuid;
use diesel::Insertable;
use crate::schema::users;

#[derive(Clone, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        let hash_password = hash(password.as_str(), DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        NewUser {
            username,
            email,
            password: hash_password,
            unique_id: uuid,
        }
    }
}