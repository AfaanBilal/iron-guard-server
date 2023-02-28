use rocket::serde::Serialize;
/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use serde_json::json;

use crate::ErrorResponder;

pub mod auth;
pub mod categories;
pub mod items;
pub mod users;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseList<T> {
    total: usize,
    results: Vec<T>,
}

pub fn success() -> Result<String, ErrorResponder> {
    Ok(json!({ "status": "success" }).to_string())
}
