/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use std::time::SystemTime;

use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    *,
};
use sea_orm::{prelude::DateTimeUtc, *};
use uuid::Uuid;

use super::{auth::AuthenticatedUser, not_found, success, ErrorResponder, ResponseList};
use crate::entities::{item, prelude::*};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestItem<'r> {
    category_id: Option<i32>,
    name: &'r str,
    description: &'r str,
    quantity: u32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseItem {
    pub id: i32,
    pub uuid: String,
    pub category_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: u32,
}

impl From<item::Model> for ResponseItem {
    fn from(item: item::Model) -> ResponseItem {
        ResponseItem {
            id: item.id,
            uuid: item.uuid,
            category_id: item.category_id,
            name: item.name,
            description: item.description,
            quantity: item.quantity,
        }
    }
}

impl Item {
    async fn from_uuid(db: &DatabaseConnection, uuid: &str) -> Result<Option<item::Model>, DbErr> {
        Item::find()
            .filter(item::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Result<Json<ResponseList<ResponseItem>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let items = Item::find()
        .all(db)
        .await?
        .into_iter()
        .map(ResponseItem::from)
        .collect::<Vec<_>>();

    Ok(Json(ResponseList {
        total: items.len(),
        results: items,
    }))
}

#[post("/", data = "<req_item>")]
pub async fn store(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_item: Json<RequestItem<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_item = item::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        user_id: Set(user.id),
        category_id: Set(req_item.category_id),
        name: Set(req_item.name.to_owned()),
        description: Set(Some(req_item.description.to_owned())),
        quantity: Set(req_item.quantity),
        ..Default::default()
    };

    Item::insert(new_item).exec(db).await?;

    success()
}

#[get("/<uuid>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
) -> Result<Json<ResponseItem>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let item = match Item::from_uuid(db, uuid).await? {
        Some(i) => i,
        None => return Err(not_found()),
    };

    Ok(Json(ResponseItem::from(item)))
}

#[put("/<uuid>", data = "<req_item>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
    req_item: Json<RequestItem<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let mut item: item::ActiveModel = match Item::from_uuid(db, uuid).await? {
        Some(i) => i.into(),
        None => return Err(not_found()),
    };

    item.category_id = Set(req_item.category_id);
    item.name = Set(req_item.name.to_owned());
    item.description = Set(Some(req_item.description.to_owned()));
    item.quantity = Set(req_item.quantity);

    item.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    item.update(db).await?;

    success()
}

#[delete("/<uuid>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let item = match Item::from_uuid(db, uuid).await? {
        Some(i) => i,
        None => return Err(not_found()),
    };

    item.delete(db).await?;

    success()
}
