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

async fn index_get(tera: web::Data<Tera>, identificator: Identity) -> impl Responder {
    let mut data = Context::new();

    if let Some(id) = identificator.identity() {
        data.insert("link1", "/logout");
        data.insert("link_text1", "Logout");
        data.insert("link2", "/");
        data.insert("link_text2", &id);

        return HttpResponse::Ok().body(tera.render("index.hbs", &data).unwrap());
    }

    data.insert("link1", "/signup");
    data.insert("link_text1", "Sign Up");
    data.insert("link2", "/signin");
    data.insert("link_text2", "Sign In");

    HttpResponse::Ok().body(tera.render("index.hbs", &data).unwrap())
}

async fn signup_get(tera: web::Data<Tera>) -> impl Responder {
    HttpResponse::Ok().body(tera.render("signup.hbs", &Context::new()).unwrap())
}

async fn signup_post(
    tera: web::Data<Tera>,
    data: web::Form<models::NewRegistratedUser>,
    identificator: Identity,
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
        let session_token = String::from(&data.username);
        identificator.remember(session_token);

        let mut index_data = Context::new();

        index_data.insert("link1", "/logout");
        index_data.insert("link_text1", "Logout");
        index_data.insert("link2", "/");
        index_data.insert("link_text2", &identificator.identity().unwrap());

        return HttpResponse::SeeOther().header("Location", "/").finish();
    }
}

async fn signin_get(tera: web::Data<Tera>, identificator: Identity) -> impl Responder {
    if let Some(_) = identificator.identity() {
        return HttpResponse::Ok().body("Already logged in.");
    }

    HttpResponse::Ok().body(tera.render("signin.hbs", &Context::new()).unwrap())
}

async fn signin_post(
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

                let mut index_data = Context::new();

                index_data.insert("link1", "/logout");
                index_data.insert("link_text1", "Logout");
                index_data.insert("link2", "/");
                index_data.insert("link_text2", &identificator.identity().unwrap());

                return HttpResponse::SeeOther().header("Location", "/").finish();
            } else {
                let mut error = Context::new();
                error.insert("err", "Incorrect login or password");
                HttpResponse::Ok().body(tera.render("signin_error.hbs", &error).unwrap())
            }
        }
        Err(_) => {
            let mut error = Context::new();
            error.insert("err", "Incorrect login or password");
            HttpResponse::Ok().body(tera.render("signin_error.hbs", &error).unwrap())
        }
    }
}

async fn logout(identificator: Identity) -> impl Responder {
    if let Some(_) = identificator.identity() {
        identificator.forget();
    }

    HttpResponse::SeeOther().header("Location", "/").finish()
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
            .route("/", web::get().to(index_get))
            .route("/signup", web::get().to(signup_get))
            .route("/signup", web::post().to(signup_post))
            .route("/signin", web::get().to(signin_get))
            .route("/signin", web::post().to(signin_post))
            .route("/logout", web::to(logout))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
