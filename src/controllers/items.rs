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
    *,
};
use sea_orm::{prelude::DateTimeUtc, *};
use uuid::Uuid;

use super::{
    auth::AuthenticatedUser, not_found, success, users::ResponseUser, ErrorResponder, ResponseList,
};
use crate::entities::{item, prelude::*};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestItem<'r> {
    category_id: Option<i32>,
    name: &'r str,
    description: Option<String>,
    quantity: u32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseItem {
    pub uuid: String,
    pub category_uuid: Option<String>,
    pub user: Option<ResponseUser>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: u32,
}

impl From<&item::Model> for ResponseItem {
    fn from(item: &item::Model) -> ResponseItem {
        ResponseItem {
            uuid: item.uuid.to_owned(),
            category_uuid: None,
            user: None,
            name: item.name.to_owned(),
            description: item.description.to_owned(),
            quantity: item.quantity,
        }
    }
}

impl Item {
    pub async fn from_uuid(
        db: &DatabaseConnection,
        uuid: &str,
    ) -> Result<Option<item::Model>, DbErr> {
        Item::find()
            .filter(item::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }

    pub async fn latest(db: &DatabaseConnection, count: u64) -> Result<Vec<ResponseItem>, DbErr> {
        Ok(Item::find()
            .order_by_desc(item::Column::UpdatedAt)
            .limit(count)
            .all(db)
            .await?
            .iter()
            .map(ResponseItem::from)
            .collect::<Vec<_>>())
    }
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Result<Json<ResponseList<ResponseItem>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let items = Item::find()
        .order_by_desc(item::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
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

    Item::insert(item::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        user_id: Set(user.id),
        category_id: Set(req_item.category_id),
        name: Set(req_item.name.to_owned()),
        description: Set(req_item.description.to_owned()),
        quantity: Set(req_item.quantity),
        ..Default::default()
    })
    .exec(db)
    .await?;

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

    let user = item.find_related(User).one(db).await?.unwrap();

    let mut response = ResponseItem::from(&item);

    if let Some(category_id) = item.category_id {
        response.category_uuid = Some(
            Category::find_by_id(category_id)
                .one(db)
                .await?
                .unwrap()
                .uuid,
        );
    }

    response.user = Some(ResponseUser::from(user));

    Ok(Json(response))
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
    item.description = Set(req_item.description.to_owned());
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
