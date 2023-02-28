/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    *,
};
use sea_orm::*;
use uuid::Uuid;

use super::{success, ErrorResponder, ResponseList};
use crate::entities::{prelude::*, user};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestUser<'r> {
    firstname: &'r str,
    lastname: &'r str,
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseUser {
    id: i32,
    uuid: String,
    firstname: Option<String>,
    lastname: Option<String>,
    email: Option<String>,
}

impl From<user::Model> for ResponseUser {
    fn from(user: user::Model) -> ResponseUser {
        ResponseUser {
            id: user.id,
            uuid: user.uuid,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
        }
    }
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
) -> Result<Json<ResponseList<ResponseUser>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let users = User::find()
        .all(db)
        .await?
        .into_iter()
        .map(ResponseUser::from)
        .collect::<Vec<_>>();

    Ok(Json(ResponseList {
        total: users.len(),
        results: users,
    }))
}

#[post("/", data = "<req_user>")]
pub async fn store(
    db: &State<DatabaseConnection>,
    req_user: Json<RequestUser<'_>>,
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

    success()
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<ResponseUser>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = match User::find_by_id(id).one(db).await? {
        Some(u) => u,
        None => return Err("404".into()),
    };

    Ok(Json(ResponseUser::from(user)))
}

#[put("/<id>", data = "<req_user>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    id: i32,
    req_user: Json<RequestUser<'_>>,
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

    success()
}

#[delete("/<id>")]
pub async fn delete(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    User::delete_by_id(id).exec(db).await?;

    success()
}
