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
    entities::{item, prelude::*},
    ErrorResponder,
};

use super::success;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqItem<'r> {
    category_id: Option<i32>,
    name: &'r str,
    description: &'r str,
    quantity: u32,
}

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let items = Item::find()
        .all(db)
        .await?
        .into_iter()
        .map(|i| json!({ "id": i.id, "name": i.name, "quantity": i.quantity }))
        .collect::<Vec<_>>();

    Ok(json!({ "items": items, "total": items.len() }).to_string())
}

#[post("/", data = "<req_item>")]
pub async fn store(
    db: &State<DatabaseConnection>,
    req_item: Json<ReqItem<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_item = item::ActiveModel {
        uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
        category_id: ActiveValue::Set(req_item.category_id),
        name: ActiveValue::Set(Some(req_item.name.to_owned())),
        description: ActiveValue::Set(Some(req_item.description.to_owned())),
        quantity: ActiveValue::Set(Some(req_item.quantity)),
        ..Default::default()
    };

    Item::insert(new_item).exec(db).await?;

    Ok(success())
}

#[get("/<id>")]
pub async fn show(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let item = Item::find_by_id(id).one(db).await?;

    if let Some(item) = item {
        Ok(json!({ "id": item.id, "name": item.name }).to_string())
    } else {
        Err(format!("404 No item found.").into())
    }
}

#[put("/<id>", data = "<req_item>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    id: i32,
    req_item: Json<ReqItem<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let item = item::ActiveModel {
        id: ActiveValue::Set(id),
        category_id: ActiveValue::Set(req_item.category_id),
        name: ActiveValue::Set(Some(req_item.name.to_owned())),
        description: ActiveValue::Set(Some(req_item.description.to_owned())),
        quantity: ActiveValue::Set(Some(req_item.quantity)),
        ..Default::default()
    };

    item.update(db).await?;

    Ok(success())
}

#[delete("/<id>")]
pub async fn delete(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    Item::delete_by_id(id).exec(db).await?;

    Ok(success())
}
