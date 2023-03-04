/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */

#[macro_use]
extern crate rocket;
use migrator::Migrator;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
use sea_orm_migration::prelude::*;

mod controllers;
mod db;
mod entities;
mod migrator;

pub struct Config {
    secret: String,
    db_type: String,
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl Config {
    pub fn make() -> Config {
        Config {
            secret: std::env::var("IRON_GUARD_SECRET").unwrap_or("test".to_string()),
            db_type: std::env::var("IRON_GUARD_DB_TYPE").unwrap_or("mysql".to_string()),
            db_host: std::env::var("IRON_GUARD_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("IRON_GUARD_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("IRON_GUARD_DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("IRON_GUARD_DB_PASSWORD").unwrap_or("".to_string()),
            db_database: std::env::var("IRON_GUARD_DB_DATABASE")
                .unwrap_or("iron_guard".to_string()),
        }
    }
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, PATCH, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Iron Guard by Afaan Bilal (https://afaan.dev)"
}

#[catch(400)]
fn bad_request() -> &'static str {
    "400 Bad Request"
}

#[catch(401)]
fn unauthorized() -> &'static str {
    "401 Unauthorized"
}

#[catch(404)]
fn not_found() -> &'static str {
    "404 Not Found"
}

#[options("/<_..>")]
fn options() -> &'static str {
    ""
}

#[launch]
async fn rocket() -> _ {
    let config = Config::make();

    let db = match db::connect(&config).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    match Migrator::up(&db, None).await {
        Err(err) => panic!("{}", err),
        Ok(_) => 0,
    };

    rocket::build()
        .attach(CORS)
        .manage(config)
        .manage(db)
        .register("/", catchers![bad_request, unauthorized, not_found])
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount("/auth", routes![controllers::auth::sign_in])
        .mount("/dashboard", routes![controllers::dashboard::index])
        .mount(
            "/me",
            routes![controllers::me::index, controllers::me::update],
        )
        .mount(
            "/inventory",
            routes![
                controllers::inventory::index,
                controllers::inventory::by_category
            ],
        )
        .mount(
            "/users",
            routes![
                controllers::users::index,
                controllers::users::store,
                controllers::users::show,
                controllers::users::update,
                controllers::users::delete
            ],
        )
        .mount(
            "/categories",
            routes![
                controllers::categories::index,
                controllers::categories::store,
                controllers::categories::show,
                controllers::categories::update,
                controllers::categories::delete
            ],
        )
        .mount(
            "/items",
            routes![
                controllers::items::index,
                controllers::items::store,
                controllers::items::show,
                controllers::items::update,
                controllers::items::delete
            ],
        )
}

#[cfg(test)]
mod test;
