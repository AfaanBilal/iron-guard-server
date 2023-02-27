/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */

#[macro_use]
extern crate rocket;

mod controllers;

#[get("/")]
fn index() -> &'static str {
    "Iron Guard"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
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
