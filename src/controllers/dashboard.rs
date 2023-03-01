/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use rocket::{
    serde::{json::Json, Serialize},
    *,
};
use sea_orm::*;

use super::{
    auth::{AuthenticatedUser, Role},
    categories::ResponseCategory,
    items::ResponseItem,
    users::ResponseUser,
    ErrorResponder,
};
use crate::entities::prelude::*;

const LATEST_COUNT: u64 = 5;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseDashboard {
    count_users: usize,
    count_categories: usize,
    count_items: usize,

    latest_users: Vec<ResponseUser>,
    latest_categories: Vec<ResponseCategory>,
    latest_items: Vec<ResponseItem>,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<Json<ResponseDashboard>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let mut users = vec![];
    let mut user_count = 0;

    if user.role == Role::Admin {
        users = User::latest(db, LATEST_COUNT).await?;
        user_count = User::find().count(db).await?;
    }

    Ok(Json(ResponseDashboard {
        count_users: user_count,
        count_categories: Category::find().count(db).await?,
        count_items: Item::find().count(db).await?,

        latest_users: users,
        latest_categories: Category::latest(db, LATEST_COUNT).await?,
        latest_items: Item::latest(db, LATEST_COUNT).await?,
    }))
}
