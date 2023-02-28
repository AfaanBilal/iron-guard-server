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
    entities::{category, prelude::*},
    ErrorResponder,
};

use super::success;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqCategory<'r> {
    name: &'r str,
    description: &'r str,
    parent_id: Option<i32>,
}

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let categories = Category::find()
        .all(db)
        .await?
        .into_iter()
        .map(|c| json!({ "id": c.id, "name": c.name }))
        .collect::<Vec<_>>();

    Ok(json!({ "categories": categories, "total": categories.len() }).to_string())
}

#[post("/", data = "<req_category>")]
pub async fn store(
    db: &State<DatabaseConnection>,
    req_category: Json<ReqCategory<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_category = category::ActiveModel {
        uuid: ActiveValue::Set(Uuid::new_v4().to_string()),
        name: ActiveValue::Set(Some(req_category.name.to_owned())),
        description: ActiveValue::Set(Some(req_category.description.to_owned())),
        parent_id: ActiveValue::Set(req_category.parent_id),
        ..Default::default()
    };

    Category::insert(new_category).exec(db).await?;

    Ok(success())
}

#[get("/<id>")]
pub async fn show(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let category = Category::find_by_id(id).one(db).await?;

    if let Some(category) = category {
        Ok(json!({ "id": category.id, "name": category.name }).to_string())
    } else {
        Err(format!("404 No category found.").into())
    }
}

#[put("/<id>", data = "<req_category>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    id: i32,
    req_category: Json<ReqCategory<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = category::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set(Some(req_category.name.to_owned())),
        description: ActiveValue::Set(Some(req_category.description.to_owned())),
        parent_id: ActiveValue::Set(req_category.parent_id),
        ..Default::default()
    };

    user.update(db).await?;

    Ok(success())
}

#[delete("/<id>")]
pub async fn delete(db: &State<DatabaseConnection>, id: i32) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    Category::delete_by_id(id).exec(db).await?;

    Ok(success())
}
