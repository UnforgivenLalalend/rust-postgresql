use diesel::{pg::PgConnection, RunQueryDsl};

pub fn insert_user<'a>(
    connection: &PgConnection,
    username: &'a str,
    password: &'a str,
) -> super::super::models::User {
    let new_user = super::super::models::NewUser {
        username: username,
        password: &super::password_hasher::sha256_hasher(password),
    };

    diesel::insert_into(super::super::schema::users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error adding new user")
}
