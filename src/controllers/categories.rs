/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use std::time::SystemTime;

use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    *, http::Status,
};
use sea_orm::{prelude::DateTimeUtc, *};
use uuid::Uuid;

use super::{
    auth::AuthenticatedUser, not_found, success, users::ResponseUser, ErrorResponder, ResponseList, Response,
};
use crate::entities::{category, prelude::*};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestCategory<'r> {
    name: &'r str,
    description: Option<String>,
    parent_uuid: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseCategory {
    uuid: String,
    name: String,
    description: Option<String>,
    parent: Option<Box<ResponseCategory>>,
    item_count: Option<usize>,
    user: Option<ResponseUser>,
}

impl From<&category::Model> for ResponseCategory {
    fn from(category: &category::Model) -> ResponseCategory {
        ResponseCategory {
            uuid: category.uuid.to_owned(),
            name: category.name.to_owned(),
            description: category.description.to_owned(),
            parent: None,
            item_count: None,
            user: None,
        }
    }
}

impl Category {
    pub async fn from_uuid(
        db: &DatabaseConnection,
        uuid: &str,
    ) -> Result<Option<category::Model>, DbErr> {
        Category::find()
            .filter(category::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }

    pub async fn latest(
        db: &DatabaseConnection,
        count: u64,
    ) -> Result<Vec<ResponseCategory>, DbErr> {
        Ok(Category::find()
            .order_by_desc(category::Column::UpdatedAt)
            .limit(count)
            .all(db)
            .await?
            .iter()
            .map(ResponseCategory::from)
            .collect::<Vec<_>>())
    }
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Result<Json<ResponseList<ResponseCategory>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let categories = Category::find()
        .order_by_desc(category::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(ResponseCategory::from)
        .collect::<Vec<_>>();

    Ok(Json(ResponseList {
        total: categories.len(),
        results: categories,
    }))
}

#[post("/", data = "<req_category>")] // FIXME: send 201 CREATED
pub async fn store(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_category: Json<RequestCategory<'_>>,
) -> Response {
    let db = db as &DatabaseConnection;

    let mut parent: Option<i32> = None;
    if let Some(parent_uuid) = req_category.parent_uuid.to_owned() {
        if let Some(p) = Category::from_uuid(db, parent_uuid.as_str()).await? {
            parent = Some(p.id);
        }
    }

    Category::insert(category::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        user_id: Set(user.id),
        name: Set(req_category.name.to_owned()),
        description: Set(req_category.description.to_owned()),
        parent_id: Set(parent),
        ..Default::default()
    })
    .exec(db)
    .await?;

    success(Status::Created)
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

    let mut response = ResponseCategory::from(&category);

    response.user = Some(ResponseUser::from(
        category.find_related(User).one(db).await?.unwrap(),
    ));

    response.item_count = Some(category.find_related(Item).count(db).await?);

    if let Some(parent_id) = category.parent_id {
        response.parent = Some(Box::new(ResponseCategory::from(
            &Category::find_by_id(parent_id).one(db).await?.unwrap(),
        )));
    }

    Ok(Json(response))
}

#[put("/<uuid>", data = "<req_category>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
    req_category: Json<RequestCategory<'_>>,
) -> Response {
    let db = db as &DatabaseConnection;

    let mut category: category::ActiveModel = match Category::from_uuid(db, uuid).await? {
        Some(c) => c.into(),
        None => return Err(not_found()),
    };

    let mut parent: Option<i32> = None;
    if let Some(parent_uuid) = req_category.parent_uuid.to_owned() {
        if let Some(p) = Category::from_uuid(db, parent_uuid.as_str()).await? {
            parent = Some(p.id);
        }
    }

    category.name = Set(req_category.name.to_owned());
    category.description = Set(req_category.description.to_owned());
    category.parent_id = Set(parent);

    category.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    category.update(db).await?;

    success(Status::Ok)
}

#[delete("/<uuid>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: &str,
) -> Response {
    let db = db as &DatabaseConnection;

    let category = match Category::from_uuid(db, uuid).await? {
        Some(c) => c,
        None => return Err(not_found()),
    };

    category.delete(db).await?;

    success(Status::Ok)
}
