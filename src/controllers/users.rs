/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use bcrypt::{hash, DEFAULT_COST};
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    *,
};
use sea_orm::*;
use uuid::Uuid;

use super::{
    admin_required,
    auth::{AuthenticatedUser, Role},
    not_found, success, ErrorResponder, ResponseList,
};
use crate::entities::{prelude::*, user};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestUser<'r> {
    role: &'r str,
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
    role: String,
    firstname: String,
    lastname: String,
    email: String,
}

impl From<user::Model> for ResponseUser {
    fn from(user: user::Model) -> ResponseUser {
        ResponseUser {
            id: user.id,
            uuid: user.uuid,
            role: user.role,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
        }
    }
}

impl User {
    async fn from_uuid(db: &DatabaseConnection, uuid: &str) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<Json<ResponseList<ResponseUser>>, ErrorResponder> {
    if user.role != Role::Admin {
        return Err(admin_required());
    }

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
    user: AuthenticatedUser,
    req_user: Json<RequestUser<'_>>,
) -> Result<String, ErrorResponder> {
    if user.role != Role::Admin {
        return Err(admin_required());
    }

    let db = db as &DatabaseConnection;

    let new_user = user::ActiveModel {
        uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
        role: ActiveValue::Set(req_user.role.to_owned()),
        firstname: ActiveValue::Set(req_user.firstname.to_owned()),
        lastname: ActiveValue::Set(req_user.lastname.to_owned()),
        email: ActiveValue::Set(req_user.email.to_owned()),
        password: ActiveValue::Set(hash(req_user.password, DEFAULT_COST).unwrap()),
        ..Default::default()
    };

    User::insert(new_user).exec(db).await?;

    success()
}

#[get("/<uuid>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    uuid: &str,
) -> Result<Json<ResponseUser>, ErrorResponder> {
    if user.role != Role::Admin {
        return Err(admin_required());
    }

    let db = db as &DatabaseConnection;

    let user = match User::from_uuid(db, uuid).await? {
        Some(u) => u,
        None => return Err(not_found()),
    };

    Ok(Json(ResponseUser::from(user)))
}

#[put("/<uuid>", data = "<req_user>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    uuid: &str,
    req_user: Json<RequestUser<'_>>,
) -> Result<String, ErrorResponder> {
    if user.role != Role::Admin {
        return Err(admin_required());
    }

    let db = db as &DatabaseConnection;

    let user = match User::from_uuid(db, uuid).await? {
        Some(u) => u,
        None => return Err(not_found()),
    };

    let user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        role: ActiveValue::Set(req_user.role.to_owned()),
        firstname: ActiveValue::Set(req_user.firstname.to_owned()),
        lastname: ActiveValue::Set(req_user.lastname.to_owned()),
        email: ActiveValue::Set(req_user.email.to_owned()),
        password: match req_user.password {
            "" => ActiveValue::NotSet,
            _ => ActiveValue::Set(hash(req_user.password, DEFAULT_COST).unwrap()),
        },
        ..Default::default()
    };

    user.update(db).await?;

    success()
}

#[delete("/<uuid>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    uuid: &str,
) -> Result<String, ErrorResponder> {
    if user.role != Role::Admin {
        return Err(admin_required());
    }

    let db = db as &DatabaseConnection;

    let user = match User::from_uuid(db, uuid).await? {
        Some(u) => u,
        None => return Err(not_found()),
    };

    user.delete(db).await?;

    success()
}
