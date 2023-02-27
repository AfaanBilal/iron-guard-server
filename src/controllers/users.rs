/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use rocket::{
    serde::{json::Json, Deserialize},
    *,
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;

use crate::{
    entities::{prelude::*, user},
    ErrorResponder,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqUser<'r> {
    firstname: &'r str,
    lastname: &'r str,
    email: &'r str,
    password: &'r str,
}

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let users = User::find()
        .all(db)
        .await?
        .into_iter()
        .map(|u| json!({ "uuid": u.uuid, "id": u.id, "firstname": u.firstname }))
        .collect::<Vec<_>>();

    Ok(json!({ "users": users, "total": users.len() }).to_string())
}

#[post("/", data = "<req_user>")]
pub async fn store(
    db: &State<DatabaseConnection>,
    req_user: Json<ReqUser<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_user = user::ActiveModel {
        uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
        firstname: ActiveValue::Set(Some(req_user.firstname.to_owned())),
        lastname: ActiveValue::Set(Some(req_user.lastname.to_owned())),
        email: ActiveValue::Set(Some(req_user.email.to_owned())),
        password: ActiveValue::Set(Some(req_user.password.to_owned())),
        ..Default::default()
    };

    User::insert(new_user).exec(db).await?;

    Ok(json!({ "status": "success" }).to_string())
}

#[get("/<id>")]
pub async fn show(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = User::find_by_id(id).one(db).await?;

    Ok(if let Some(user) = user {
        json!({ "id": user.id, "firstname": user.firstname, "lastname": user.lastname }).to_string()
    } else {
        return Err(format!("404 No user found.").into());
    })
}

#[put("/<id>", data = "<req_user>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    id: i32,
    req_user: Json<ReqUser<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = user::ActiveModel {
        id: ActiveValue::Set(id),
        firstname: ActiveValue::Set(Some(req_user.firstname.to_owned())),
        lastname: ActiveValue::Set(Some(req_user.lastname.to_owned())),
        email: ActiveValue::Set(Some(req_user.email.to_owned())),
        password: ActiveValue::Set(Some(req_user.password.to_owned())),
        ..Default::default()
    };

    user.update(db).await?;

    Ok("Updated".to_owned())
}

#[delete("/<id>")]
pub async fn delete(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    User::delete_by_id(id).exec(db).await?;

    Ok("Deleted".to_owned())
}
