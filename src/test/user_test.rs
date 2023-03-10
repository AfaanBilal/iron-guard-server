/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use super::super::rocket;
#[allow(unused_imports)]
use crate::test::utils::{create_test_user, create_test_admin, delete_test_user, get_auth_header, get_client};
use rocket::http::Status;
use serde_json::json;

#[async_test]
async fn should_403() {
    // create_test_user().await;

    let client = get_client().await;

    let response = client
        .get("/users")
        .header(get_auth_header(false).await)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Forbidden);
}

#[async_test]
async fn should_list_users() {
    // create_test_admin().await;

    let client = get_client().await;

    let response = client
        .get("/users")
        .header(get_auth_header(true).await)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let r = response.into_string().await;
    assert!(r.is_some());
    let r = r.unwrap();

    assert!(r.contains("results"));
}

#[async_test]
async fn should_not_add_user() {
    let client = get_client().await;

    let response = client
        .post("/users")
        .header(get_auth_header(true).await)
        .body(json!({ "email": "test@example.net" }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[async_test]
async fn should_add_user() {
    let client = get_client().await;

    let response = client
         .post("/users")
         .header(get_auth_header(true).await)
         .body(json!({ "email": "test@example.net", "password": "test-password", "firstname": "Test A", "lastname": "User", "role": "user" }).to_string())
         .dispatch()
         .await;

    assert_eq!(response.status(), Status::Created);

    delete_test_user().await;
}
