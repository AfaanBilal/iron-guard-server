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

use super::{
    auth::AuthenticatedUser, items::ResponseItem, not_found, success, users::ResponseUser,
    ErrorResponder, ResponseList,
};
use crate::entities::{category, prelude::*};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestCategory<'r> {
    name: &'r str,
    description: Option<String>,
    parent_id: Option<i32>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseCategory {
    uuid: String,
    name: String,
    description: Option<String>,
    parent_id: Option<i32>,
    user: Option<ResponseUser>,
    items: Vec<ResponseItem>,
    children: Vec<ResponseCategory>,
}

impl From<category::Model> for ResponseCategory {
    fn from(category: category::Model) -> ResponseCategory {
        ResponseCategory {
            uuid: category.uuid,
            name: category.name,
            description: category.description,
            parent_id: category.parent_id,
            user: None,
            items: vec![],
            children: vec![],
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
    user: AuthenticatedUser,
    req_category: Json<RequestCategory<'_>>,
) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_category = category::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        user_id: Set(user.id),
        name: Set(req_category.name.to_owned()),
        description: Set(req_category.description.to_owned()),
        parent_id: Set(req_category.parent_id),
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

    let children = Category::find()
        .filter(category::Column::ParentId.eq(category.id))
        .all(db)
        .await?
        .into_iter()
        .map(ResponseCategory::from)
        .collect::<Vec<_>>();

    let user = category.find_related(User).one(db).await?.unwrap();

    let mut response = ResponseCategory::from(category);

    response.user = Some(ResponseUser::from(user));
    response.items = items;
    response.children = children;

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

    let mut category: category::ActiveModel = match Category::from_uuid(db, uuid).await? {
        Some(c) => c.into(),
        None => return Err(not_found()),
    };

    category.name = Set(req_category.name.to_owned());
    category.description = Set(req_category.description.to_owned());
    category.parent_id = Set(req_category.parent_id);

    category.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    category.update(db).await?;

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
