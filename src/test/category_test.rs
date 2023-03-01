/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use super::super::rocket;
use crate::test::utils::{delete_test_category, get_auth_header, get_client};
use rocket::http::Status;
use serde_json::json;

#[async_test]
async fn should_list_categories() {
    let client = get_client().await;

    let response = client
        .get("/categories")
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
async fn should_not_add_category() {
    let client = get_client().await;

    let response = client
        .post("/categories")
        .header(get_auth_header(true).await)
        .body(json!({}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[async_test]
async fn should_add_category() {
    let client = get_client().await;

    let response = client
        .post("/categories")
        .header(get_auth_header(true).await)
        .body(json!({ "name": "test" }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    delete_test_category().await;
}
