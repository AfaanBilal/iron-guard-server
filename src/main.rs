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
    let db = match db::connect().await {
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
            "/auth",
            routes![controllers::auth::sign_in, controllers::auth::sign_out,],
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
