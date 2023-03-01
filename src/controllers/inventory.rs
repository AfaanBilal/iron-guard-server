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
    categories: Vec<ResponseCategory>,
    items: Vec<ResponseItem>,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Result<Json<ResponseInventory>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    Ok(Json(ResponseInventory {
        categories: Category::find()
            .filter(category::Column::ParentId.is_null())
            .order_by_desc(category::Column::UpdatedAt)
            .all(db)
            .await?
            .iter()
            .map(ResponseCategory::from)
            .collect::<Vec<_>>(),
        items: Item::find()
            .filter(item::Column::CategoryId.is_null())
            .order_by_desc(item::Column::UpdatedAt)
            .all(db)
            .await?
            .iter()
            .map(ResponseItem::from)
            .collect::<Vec<_>>(),
    }))
}
