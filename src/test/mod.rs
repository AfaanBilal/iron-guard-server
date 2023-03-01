/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use super::rocket;

use crate::test::utils::get_client;
use rocket::http::Status;

pub mod auth_test;
pub mod category_test;
pub mod inventory_test;
pub mod item_test;
pub mod user_test;

pub mod utils;

#[async_test]
async fn index() {
    let client = get_client().await;
    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await, Some("Iron Guard".into()));
}

#[async_test]
async fn should_401() {
    let client = get_client().await;
    let response = client.get("/me").dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(
        response.into_string().await,
        Some("401 Unauthorized".into())
    );
}

#[async_test]
async fn should_404() {
    let client = get_client().await;
    let response = client.get("/should-404").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.into_string().await, Some("404 Not Found".into()));
}
