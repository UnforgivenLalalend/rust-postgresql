use diesel::{pg::PgConnection, RunQueryDsl};

pub fn add_user<'a>(
    connection: &PgConnection,
    email: &'a str,
    username: &'a str,
    password: &'a str,
    date: &'a str,
) -> super::super::models::User {
    let new_user = super::super::models::NewUser {
        email,
        username,
        password,
        date,
        access_level: &false,
    };

    diesel::insert_into(super::super::schema::users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error adding new user")
}
