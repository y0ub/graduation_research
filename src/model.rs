use crate::schema::*;
use serde_derive::{Serialize, Deserialize};
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "user_info"]
pub struct Params<'a> {
    pub id: i32,
    pub name: &'a str,
    pub password: &'a str,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct NewParams {
    pub new_id: i32,
    pub new_name: String,
    pub new_pass: String,
    pub retype_new_pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    pub id: i32,
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct SecureParams {
    pub secure_password: String,
}
