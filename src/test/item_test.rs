/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use super::super::rocket;
use crate::test::utils::{delete_test_item, get_auth_header, get_client};
use rocket::http::Status;
use serde_json::json;

#[async_test]
async fn should_list_items() {
    let client = get_client().await;

    let response = client
        .get("/items")
        .header(get_auth_header(false).await)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let r = response.into_string().await;
    assert!(r.is_some());
    let r = r.unwrap();

    assert!(r.contains("results"));
}

#[async_test]
async fn should_not_add_item() {
    let client = get_client().await;

    let response = client
        .post("/items")
        .header(get_auth_header(true).await)
        .body(json!({}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[async_test]
async fn should_add_item() {
    let client = get_client().await;

    let response = client
        .post("/items")
        .header(get_auth_header(true).await)
        .body(json!({ "name": "test", "quantity": 5 }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    delete_test_item().await;
}
