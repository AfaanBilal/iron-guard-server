/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */

#[get("/")]
pub fn index() -> &'static str {
    "item list"
}

#[post("/")]
pub fn store() -> &'static str {
    "item store"
}

#[get("/<item>")]
pub fn show(item: u32) -> &'static str {
    "item show"
}

#[put("/<item>")]
pub fn update(item: u32) -> &'static str {
    "item update"
}

#[delete("/<item>")]
pub fn delete(item: u32) -> &'static str {
    "item delete"
}
