/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use super::super::rocket;
use crate::test::get_client;
use rocket::http::Status;
use serde_json::json;

#[async_test]
async fn should_400() {
    let client = get_client().await;
    let response = client.post("/auth/sign-in").dispatch().await;

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().await, Some("400 Bad Request".into()));
}

#[async_test]
async fn should_reject_sign_in_422_missing_password() {
    let client = get_client().await;
    let response = client
        .post("/auth/sign-in")
        .body(json!({"email": "test@example.com"}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[async_test]
async fn should_reject_sign_in_422_missing_email() {
    let client = get_client().await;
    let response = client
        .post("/auth/sign-in")
        .body(json!({"password": "test-password"}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[async_test]
async fn should_reject_sign_in_401_invalid_credentials() {
    let client = get_client().await;
    let response = client
        .post("/auth/sign-in")
        .body(json!({"email": "test@example.com", "password": "test-password"}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);

    let r = response.into_string().await;
    assert!(r.is_some());
    assert!(r.unwrap().contains("Invalid credentials"));
}

#[async_test]
async fn should_accept_sign_in() {
    // create_test_user();

    let client = get_client().await;
    let response = client
        .post("/auth/sign-in")
        .body(json!({"email": "user@example.com", "password": "test1234"}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let r = response.into_string().await;
    assert!(r.is_some());

    let r = r.unwrap();
    assert!(r.contains("success"));
    assert!(r.contains("token"));
}
