#[macro_use]
extern crate diesel;

pub mod connection;
pub mod models;
pub mod schema;

pub mod commands;

fn main() {
    let connection = connection::establish_connection();

    commands::update_ids::update_ids(&connection);

    commands::add_user::add_user(&connection, "user", "hello");
    // match commands::delete_user::delete_user(&connection, 4321) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         println!("{}", err);
    //         std::process::exit(1)
    //     }
    // }
    // println!("{:#?}", commands::show_users::show_users(&connection));

    commands::update_ids::update_ids(&connection);
}
