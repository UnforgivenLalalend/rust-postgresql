#[derive(Queryable, PartialEq, Debug, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
