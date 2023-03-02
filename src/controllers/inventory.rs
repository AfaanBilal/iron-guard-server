/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use rocket::{
    serde::{json::Json, Serialize},
    *,
};
use sea_orm::*;

use super::{
    auth::AuthenticatedUser, categories::ResponseCategory, items::ResponseItem, ErrorResponder,
};
use crate::entities::{category, item, prelude::*};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseInventory {
    category: Option<ResponseCategory>,
    categories: Vec<ResponseCategory>,
    items: Vec<ResponseItem>,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<Json<ResponseInventory>, ErrorResponder> {
    by_category(db, user, None).await
}

#[get("/<uuid>")]
pub async fn by_category(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    uuid: Option<String>,
) -> Result<Json<ResponseInventory>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let mut response = ResponseInventory {
        category: None,
        categories: vec![],
        items: vec![],
    };

    let mut f1 = category::Column::ParentId.is_null();
    let mut f2 = item::Column::CategoryId.is_null();
    if let Some(uuid) = uuid {
        let c = Category::from_uuid(db, uuid.as_str()).await?;

        if let Some(c) = c {
            response.category = Some(ResponseCategory::from(&c));
            f1 = category::Column::ParentId.eq(c.id);
            f2 = item::Column::CategoryId.eq(c.id);
        }
    }

    response.categories = Category::find()
        .filter(f1)
        .order_by_desc(category::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(ResponseCategory::from)
        .collect::<Vec<_>>();

    response.items = Item::find()
        .filter(f2)
        .order_by_desc(item::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(ResponseItem::from)
        .collect::<Vec<_>>();

    Ok(Json(response))
}
