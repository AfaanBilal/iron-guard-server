/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */

#[get("/")]
pub fn index() -> &'static str {
    "User list"
}

#[post("/")]
pub fn store() -> &'static str {
    "User store"
}

#[get("/<user>")]
pub fn show(user: u32) -> &'static str {
    "User show"
}

#[put("/<user>")]
pub fn update(user: u32) -> &'static str {
    "User update"
}

#[delete("/<user>")]
pub fn delete(user: u32) -> &'static str {
    "User delete"
}
