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

pub mod auth_test;
pub mod user_test;

#[allow(dead_code)]
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

#[allow(dead_code)]
async fn create_test_admin() {
    let db = match db::connect().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let new_user = user::ActiveModel {
        uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
        role: ActiveValue::Set("admin".to_string()),
        firstname: ActiveValue::Set("Test".to_string()),
        lastname: ActiveValue::Set("Admin".to_string()),
        email: ActiveValue::Set("admin@example.com".to_string()),
        password: ActiveValue::Set(hash("admin1234", DEFAULT_COST).unwrap()),
        ..Default::default()
    };

    match User::insert(new_user).exec(&db).await {
        Err(err) => panic!("{}", err.to_string()),
        _ => 0,
    };
}

async fn delete_test_user() {
    let db = match db::connect().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let user = User::find()
        .filter(user::Column::Email.eq("test@example.net"))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    user.delete(&db).await.unwrap();
}

pub async fn get_client() -> Client {
    Client::tracked(rocket().await).await.unwrap()
}

pub async fn get_token(admin: bool) -> String {
    let client = get_client().await;

    let mut body = json!({"email": "user@example.com", "password": "test1234"});
    if admin {
        body = json!({"email": "admin@example.com", "password": "admin1234"});
    }

    let response = client
        .post("/auth/sign-in")
        .body(body.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let r = response.into_string().await;
    assert!(r.is_some());
    let r = r.unwrap();

    let r: ResponseSignIn = serde_json::from_str(&r).unwrap();

    r.token
}

pub async fn get_auth_header(admin: bool) -> Header<'static> {
    Header::new("token", get_token(admin).await)
}

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
    let response = client.get("/users").dispatch().await;

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
