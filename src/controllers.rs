/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use serde_json::json;

pub mod auth;
pub mod categories;
pub mod items;
pub mod users;

pub fn success() -> String {
    json!({ "status": "success" }).to_string()
}
