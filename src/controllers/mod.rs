/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use rocket::{http::Status, serde::Serialize};
use sea_orm::DbErr;
use serde_json::json;

pub mod auth;
pub mod categories;
pub mod dashboard;
pub mod inventory;
pub mod items;
pub mod me;
pub mod users;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseList<T> {
    total: usize,
    results: Vec<T>,
}

#[derive(Responder)]
pub enum ErrorResponder {
    Error((Status, String)),
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> Self {
        ErrorResponder::Error((Status::InternalServerError, err.to_string()))
    }
}

type Response = Result<(Status, String), ErrorResponder>;

pub fn success(status: Status) -> Response {
    Ok((status, json!({ "status": "success" }).to_string()))
}

pub fn error_response(status: Status, message: String) -> ErrorResponder {
    ErrorResponder::Error((
        status,
        json!({ "status": "error", "message": message }).to_string(),
    ))
}

pub fn admin_required() -> ErrorResponder {
    ErrorResponder::Error((Status::Forbidden, "Admin Required".to_string()))
}

pub fn not_found() -> ErrorResponder {
    ErrorResponder::Error((Status::NotFound, "Not Found".to_string()))
}
