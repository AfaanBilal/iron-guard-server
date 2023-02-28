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

use super::{
    auth::AuthenticatedUser, items::ResponseItem, not_found, success, ErrorResponder, ResponseList,
};
use crate::entities::{category, prelude::*};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestCategory<'r> {
    name: &'r str,
    description: &'r str,
    parent_id: Option<i32>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseCategory {
    id: i32,
    uuid: String,
    name: Option<String>,
    description: Option<String>,
    parent_id: Option<i32>,
    items: Vec<ResponseItem>,
}

impl From<category::Model> for ResponseCategory {
    fn from(category: category::Model) -> ResponseCategory {
        ResponseCategory {
            id: category.id,
            uuid: category.uuid,
            name: category.name,
            description: category.description,
            parent_id: category.parent_id,
            items: vec![],
        }
    }
}

impl Category {
    async fn from_uuid(
        db: &DatabaseConnection,
        uuid: &str,
    ) -> Result<Option<category::Model>, DbErr> {
        Category::find()
            .filter(category::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Result<Json<ResponseList<ResponseCategory>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let categories = Category::find()
        .all(db)
        .await?
        .into_iter()
        .map(ResponseCategory::from)
        .collect::<Vec<_>>();

    Ok(Json(ResponseList {
        total: categories.len(),
        results: categories,
    }))
}

#[post("/", data = "<req_category>")]
pub async fn store(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    req_category: Json<RequestCategory<'_>>,
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

    success()
}

#[get("/<uuid>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
) -> Result<Json<ResponseCategory>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let category = match Category::from_uuid(db, uuid).await? {
        Some(c) => c,
        None => return Err(not_found()),
    };

    let items = category
        .find_related(Item)
        .all(db)
        .await?
        .into_iter()
        .map(ResponseItem::from)
        .collect::<Vec<_>>();

    let mut response = ResponseCategory::from(category);
    response.items = items;

    Ok(Json(response))
}

#[put("/<uuid>", data = "<req_category>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
    req_category: Json<RequestCategory<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let category = match Category::from_uuid(db, uuid).await? {
        Some(c) => c,
        None => return Err(not_found()),
    };

    let user = category::ActiveModel {
        id: ActiveValue::Set(category.id),
        name: ActiveValue::Set(Some(req_category.name.to_owned())),
        description: ActiveValue::Set(Some(req_category.description.to_owned())),
        parent_id: ActiveValue::Set(req_category.parent_id),
        ..Default::default()
    };

    user.update(db).await?;

    success()
}

#[delete("/<uuid>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let category = match Category::from_uuid(db, uuid).await? {
        Some(c) => c,
        None => return Err(not_found()),
    };

    category.delete(db).await?;

    success()
}
