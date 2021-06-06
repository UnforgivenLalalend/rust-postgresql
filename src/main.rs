#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use tera::{Context, Tera};

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
    use schema::users::dsl::*;

    let connection = connection::establish_connection();

    let mut error = Context::new();
    let mut is_error = false;
    let email_regex = regex::Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    let username_regex = regex::Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();

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

    match users
        .filter(email.eq_all(&data.email))
        .first::<models::User>(&connection)
    {
        Ok(_) => {
            error.insert("err", "Account with simmilar email is already exists");
            is_error = true;
        }
        Err(_) => {
            match users
                .filter(username.eq_all(&data.username))
                .first::<models::User>(&connection)
            {
                Ok(_) => {
                    error.insert("err", "Account with simmilar username is already exists");
                    is_error = true;
                }
                Err(_) => {}
            }
        }
    }

    if is_error {
        HttpResponse::Ok().body(tera.render("signup_error.hbs", &error).unwrap())
    } else {
        let hashed_password = password_hasher::password_hasher(data.password.trim());

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

async fn login_get(tera: web::Data<Tera>, identificator: Identity) -> impl Responder {
    if let Some(_) = identificator.identity() {
        return HttpResponse::Ok().body("Already logged in.");
    }

    HttpResponse::Ok().body(tera.render("login.html", &Context::new()).unwrap())
}

async fn login_post(
    tera: web::Data<Tera>,
    data: web::Form<models::LoginUser>,
    identificator: Identity,
) -> impl Responder {
    use schema::users::dsl::*;
    let connection = connection::establish_connection();

    let user = users
        .filter(username.eq_all(&data.username))
        .first::<models::User>(&connection);

    match user {
        Ok(u) => {
            let password_db = u.password.split("$").collect::<Vec<&str>>();
            let salt = password_db[3];

            if u.password == password_hasher::password_hasher_with_salt(salt, &data.password) {
                let session_token = String::from(&u.username);
                identificator.remember(session_token);
                return HttpResponse::Ok().body(format!("Successfully logged as: {}", u.username));
            } else {
                let mut error = Context::new();
                error.insert("err", "Incorrect login or password");
                HttpResponse::Ok().body(tera.render("login_error.hbs", &error).unwrap())
            }
        }
        Err(_) => {
            let mut error = Context::new();
            error.insert("err", "Incorrect login or password");
            HttpResponse::Ok().body(tera.render("login_error.hbs", &error).unwrap())
        }
    }
}

async fn logout(identificator: Identity) -> impl Responder {
    identificator.forget();
    HttpResponse::Ok().body("Logged out.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
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
            .route("/logout", web::to(logout))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
