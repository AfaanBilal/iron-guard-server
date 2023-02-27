/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */

#[macro_use]
extern crate rocket;
use migrator::Migrator;
use sea_orm_migration::prelude::*;

mod controllers;
mod db;
mod entities;
mod migrator;

#[get("/")]
fn index() -> &'static str {
    "Iron Guard"
}

#[catch(404)]
fn not_found() -> &'static str {
    "404 Not Found"
}

#[launch]
async fn rocket() -> _ {
    let db = match db::set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    match Migrator::refresh(&db).await {
        Err(err) => panic!("{}", err),
        Ok(_) => 0,
    };

    rocket::build()
        .manage(db)
        .register("/", catchers![not_found])
        .mount("/", routes![index])
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
    // .mount(
    //     "/items",
    //     routes![
    //         controllers::items::index,
    //         controllers::items::store,
    //         controllers::items::show,
    //         controllers::items::update,
    //         controllers::items::delete
    //     ],
    // )
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}
