// use log::info;

#[macro_use]
extern crate diesel;

pub mod connection;
pub mod models;
pub mod schema;

pub mod commands;

fn main() {
    let connection = connection::establish_connection();

    commands::update_ids::update(&connection);

    commands::add_user::insert_user(&connection, "user", "hello");
    // match commands::delete_user::remove_user(&connection, 4321) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         info!("{}", err);
    //         std::process::exit(1)
    //     }
    // }
    // commands::show_users::display_users(&connection);

    commands::update_ids::update(&connection);
}
