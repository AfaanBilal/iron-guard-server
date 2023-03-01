/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use super::rocket;
use crate::{
    controllers::auth::ResponseSignIn,
    db,
    entities::{category, item, prelude::*, user},
    Config,
};
use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    http::{Header, Status},
    local::asynchronous::Client,
};
use sea_orm::*;
use serde_json::json;
use uuid::Uuid;

pub async fn get_db() -> DatabaseConnection {
    let db = match db::connect(&Config::make()).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    db
}

#[allow(dead_code)]
pub async fn create_test_user() {
    let db = get_db().await;

    let new_user = user::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        role: Set("user".to_string()),
        firstname: Set("Test".to_string()),
        lastname: Set("User".to_string()),
        email: Set("user@example.com".to_string()),
        password: Set(hash("test1234", DEFAULT_COST).unwrap()),
        ..Default::default()
    };

    match User::insert(new_user).exec(&db).await {
        Err(err) => panic!("{}", err.to_string()),
        _ => 0,
    };
}

#[allow(dead_code)]
pub async fn create_test_admin() {
    let db = get_db().await;

    let new_user = user::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        role: Set("admin".to_string()),
        firstname: Set("Test".to_string()),
        lastname: Set("Admin".to_string()),
        email: Set("admin@example.com".to_string()),
        password: Set(hash("admin1234", DEFAULT_COST).unwrap()),
        ..Default::default()
    };

    match User::insert(new_user).exec(&db).await {
        Err(err) => panic!("{}", err.to_string()),
        _ => 0,
    };
}

pub async fn delete_test_user() {
    let db = get_db().await;

    let user = User::find()
        .filter(user::Column::Email.eq("test@example.net"))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    user.delete(&db).await.unwrap();
}

pub async fn delete_test_category() {
    let db = get_db().await;

    let category = Category::find()
        .filter(category::Column::Name.eq("test"))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    category.delete(&db).await.unwrap();
}

pub async fn delete_test_item() {
    let db = get_db().await;

    let item = Item::find()
        .filter(item::Column::Name.eq("test"))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    item.delete(&db).await.unwrap();
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
