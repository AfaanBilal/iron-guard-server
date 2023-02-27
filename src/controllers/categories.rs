/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */

#[get("/")]
pub fn index() -> &'static str {
    "category list"
}

#[post("/")]
pub fn store() -> &'static str {
    "category store"
}

#[get("/<category>")]
pub fn show(category: u32) -> &'static str {
    "category show"
}

#[put("/<category>")]
pub fn update(category: u32) -> &'static str {
    "category update"
}

#[delete("/<category>")]
pub fn delete(category: u32) -> &'static str {
    "category delete"
}
