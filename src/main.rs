#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

use actix_files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use commands::show_users::show_users;
use regex::Regex;
use tera::{Context, Tera};

use crate::password_hasher::password_hasher;

pub mod commands;
pub mod connection;
pub mod models;
pub mod password_hasher;
pub mod schema;
pub mod update_ids;

async fn signup_get(tera: web::Data<Tera>) -> impl Responder {
    HttpResponse::Ok().body(tera.render("signup.html", &Context::new()).unwrap())
}

async fn signup_post(
    tera: web::Data<Tera>,
    data: web::Form<models::NewRegistratedUser>,
) -> impl Responder {
    let connection = connection::establish_connection();

    let mut error = Context::new();
    let mut is_error = false;
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    let username_regex = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();

    if !email_regex.is_match(data.email.trim()) {
        error.insert("err", "Incorrect email format");
        is_error = true;
    } else if !username_regex.is_match(data.username.trim()) {
        error.insert("err", "Incorrect username format");
        is_error = true;
    } else if data.password.trim().len() < 5 {
        error.insert("err", "Password is too short");
        is_error = true;
    } else if data.password.trim() != data.confirmed_password.trim() {
        error.insert("err", "Passwords are not simmilar");
        is_error = true;
    }

    let all_users = show_users(&connection);
    for user in all_users {
        if user.email == data.email.trim() {
            error.insert("err", "Account with simmilar email is already exists");
            is_error = true;
            break;
        }
    }

    if is_error {
        return HttpResponse::Ok().body(tera.render("signup_error.hbs", &error).unwrap());
    } else {
        let hashed_password =
            password_hasher(data.username.trim().clone(), data.password.trim().clone());

        commands::add_user::add_user(
            &connection,
            &data.email.trim(),
            &data.username.trim(),
            &hashed_password,
        );
        update_ids::update_ids(&connection);
        println!("{:?}", data);
        return HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username));
    }
}

async fn login_get(tera: web::Data<Tera>) -> impl Responder {
    HttpResponse::Ok().body(tera.render("login.html", &Context::new()).unwrap())
}

async fn login_post(tera: web::Data<Tera>, data: web::Form<models::LoginUser>) -> impl Responder {
    let connection = connection::establish_connection();
    let all_users = commands::show_users::show_users(&connection);

    let hashed_password =
        password_hasher(data.username.trim().clone(), data.password.trim().clone());

    for user in all_users {
        if user.username == data.username.trim() && user.password == hashed_password {
            return HttpResponse::Ok().body(format!("Successfully logged as: {}", user.username));
        }
    }

    let mut error = Context::new();
    error.insert("err", "Incorrect login or password");
    HttpResponse::Ok().body(tera.render("login_error.hbs", &error).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let connection = connection::establish_connection();

    // update_ids::update_ids(&connection);

    // commands::add_user::add_user(&connection, "user", "hello");
    // match commands::delete_user::delete_user(&connection, 1) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         println!("{}", err);
    //         std::process::exit(1)
    //     }
    // }
    // println!("{:#?}", commands::show_users::show_users(&connection));

    // update_ids::update_ids(&connection);
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
            .route("/signup", web::get().to(signup_get))
            .route("/signup", web::post().to(signup_post))
            .route("/login", web::get().to(login_get))
            .route("/login", web::post().to(login_post))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
