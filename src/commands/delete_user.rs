use crate::{commands::show_users, diesel::ExpressionMethods};
use diesel::{pg::PgConnection, QueryDsl, RunQueryDsl};

use anyhow::{anyhow, Context};

pub fn delete_user(connection: &PgConnection, id_number: i32) -> Result<usize, anyhow::Error> {
    use super::super::schema::users::dsl::*;

    let all_users = show_users::show_users(connection);
    let last_user_id = all_users[all_users.len() - 1].id;

    if id_number > last_user_id {
        return Err(anyhow!("given username id does not exist in database"));
    }

    diesel::delete(users.filter(id.eq(id_number)))
        .execute(connection)
        .context("unable to execute user deletion")
}
