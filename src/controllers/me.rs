/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use std::time::SystemTime;

use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    serde::{json::Json, Deserialize},
    *, http::Status,
};
use sea_orm::{prelude::DateTimeUtc, *};

use super::{auth::AuthenticatedUser, success, users::ResponseUser, ErrorResponder, Response};
use crate::entities::{prelude::*, user};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestMe<'r> {
    firstname: &'r str,
    lastname: &'r str,
    email: &'r str,
    password: &'r str,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<Json<ResponseUser>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = User::find_by_id(user.id).one(db).await?.unwrap();

    Ok(Json(ResponseUser::from(user)))
}

#[put("/", data = "<req_me>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_me: Json<RequestMe<'_>>,
) -> Response {
    let db = db as &DatabaseConnection;

    let mut user: user::ActiveModel = User::find_by_id(user.id).one(db).await?.unwrap().into();

    user.firstname = Set(req_me.firstname.to_owned());
    user.lastname = Set(req_me.lastname.to_owned());
    user.email = Set(req_me.email.to_owned());

    if !req_me.password.is_empty() {
        user.password = Set(hash(req_me.password, DEFAULT_COST).unwrap());
    }

    user.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    user.update(db).await?;

    success(Status::Ok)
}
