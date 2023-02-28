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
    pub name: Option<String>,
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
    _user: AuthenticatedUser,
    req_item: Json<RequestItem<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_item = item::ActiveModel {
        uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
        category_id: ActiveValue::Set(req_item.category_id),
        name: ActiveValue::Set(Some(req_item.name.to_owned())),
        description: ActiveValue::Set(Some(req_item.description.to_owned())),
        quantity: ActiveValue::Set(req_item.quantity),
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

    let item = match Item::from_uuid(db, uuid).await? {
        Some(i) => i,
        None => return Err(not_found()),
    };

    let item = item::ActiveModel {
        id: ActiveValue::Set(item.id),
        category_id: ActiveValue::Set(req_item.category_id),
        name: ActiveValue::Set(Some(req_item.name.to_owned())),
        description: ActiveValue::Set(Some(req_item.description.to_owned())),
        quantity: ActiveValue::Set(req_item.quantity),
        ..Default::default()
    };

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
