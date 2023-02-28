/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use rocket::serde::Serialize;
use sea_orm::DbErr;
use serde_json::json;

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

pub fn success() -> Result<String, ErrorResponder> {
    Ok(json!({ "status": "success" }).to_string())
}
