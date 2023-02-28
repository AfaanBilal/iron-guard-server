/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use super::rocket;
use rocket::{http::Status, local::asynchronous::Client};
use serde_json::json;

#[async_test]
async fn index() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await, Some("Iron Guard".into()));
}

#[async_test]
async fn should_401() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client.get("/users").dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(
        response.into_string().await,
        Some("401 Unauthorized".into())
    );
}

#[async_test]
async fn should_404() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client.get("/should-404").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.into_string().await, Some("404 Not Found".into()));
}

#[async_test]
async fn should_400() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client.post("/auth/sign-in").dispatch().await;

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().await, Some("400 Bad Request".into()));
}

#[async_test]
async fn should_reject_sign_in_422_missing_password() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client
        .post("/auth/sign-in")
        .body(json!({"email": "test@example.com"}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[async_test]
async fn should_reject_sign_in_422_missing_email() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client
        .post("/auth/sign-in")
        .body(json!({"password": "test-password"}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[async_test]
async fn should_reject_sign_in_401_invalid_credentials() {
    let client = Client::tracked(rocket().await).await.unwrap();
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
