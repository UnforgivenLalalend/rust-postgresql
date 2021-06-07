use serde::{Deserialize, Serialize};

#[derive(Queryable, PartialEq, Debug, Insertable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub date: String,
    pub access_level: bool,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub date: &'a str,
    pub access_level: &'a bool,
}

#[derive(Deserialize, Debug)]
pub struct NewRegistratedUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirmed_password: String,
}

#[derive(Deserialize, Debug, Queryable)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: String,
}
