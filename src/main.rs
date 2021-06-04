#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

use actix_files;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};

use serde::Deserialize;

pub mod commands;
pub mod connection;
pub mod models;
pub mod schema;
pub mod update_ids;

async fn login_page_get(tera: web::Data<Tera>) -> impl Responder {
    HttpResponse::Ok().body(tera.render("login.hbs", &Context::new()).unwrap())
}

async fn process_signup(data: web::Form<models::NewUser>) -> impl Responder {
    commands::add_user::add_user(connection, data.username, data.password);

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = connection::establish_connection();

    update_ids::update_ids(&connection);

    // commands::add_user::add_user(&connection, "user", "hello");
    // match commands::delete_user::delete_user(&connection, 1) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         println!("{}", err);
    //         std::process::exit(1)
    //     }
    // }
    println!("{:#?}", commands::show_users::show_users(&connection));

    update_ids::update_ids(&connection);

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .service(
                actix_files::Files::new(
                    "/static",
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("static"),
                )
                .show_files_listing(),
            )
            .data(tera)
            .route("/", web::get().to(process_signup))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
