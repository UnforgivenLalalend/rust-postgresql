use diesel::{pg::PgConnection, RunQueryDsl};

pub fn update(connection: &PgConnection) {
    use super::super::schema::users::dsl::*;

    let mut all_users = users
        .load::<super::super::models::User>(connection)
        .expect("Error showing users");

    diesel::delete(users).execute(connection).unwrap();

    for updated_id in 1..=all_users.len() {
        all_users[updated_id - 1].id = updated_id as i32;

        diesel::insert_into(super::super::schema::users::table)
            .values(&all_users[updated_id - 1])
            .get_result::<(i32, String, String)>(connection)
            .unwrap();
    }
}
