/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use super::rocket;
use crate::{
    controllers::auth::ResponseSignIn,
    db,
    entities::{prelude::*, user},
};
use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    http::{Header, Status},
    local::asynchronous::Client,
};
use sea_orm::ActiveValue;
use sea_orm::*;
use serde_json::json;
use uuid::Uuid;

async fn create_test_user() {
    let db = match db::connect().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let new_user = user::ActiveModel {
        uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
        role: ActiveValue::Set("user".to_string()),
        firstname: ActiveValue::Set("Test".to_string()),
        lastname: ActiveValue::Set("User".to_string()),
        email: ActiveValue::Set("user@example.com".to_string()),
        password: ActiveValue::Set(hash("test1234", DEFAULT_COST).unwrap()),
        ..Default::default()
    };

    match User::insert(new_user).exec(&db).await {
        Err(err) => panic!("{}", err.to_string()),
        _ => 0,
    };
}

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

#[async_test]
async fn should_accept_sign_in() {
    // create_test_user();

    let client = Client::tracked(rocket().await).await.unwrap();
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

#[async_test]
async fn should_403() {
    // create_test_user();

    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client
        .post("/auth/sign-in")
        .body(json!({"email": "user@example.com", "password": "test1234"}).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let r = response.into_string().await;
    assert!(r.is_some());
    let r = r.unwrap();

    let r: ResponseSignIn = serde_json::from_str(&r).unwrap();

    let response = client
        .get("/users")
        .header(Header::new("token", r.token))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Forbidden);
}
