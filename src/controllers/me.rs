/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use rocket::{serde::json::Json, *};
use sea_orm::*;

use super::{auth::AuthenticatedUser, users::ResponseUser, ErrorResponder};
use crate::entities::prelude::*;

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Result<Json<ResponseUser>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = User::find_by_id(user.id).one(db).await?.unwrap();

    Ok(Json(ResponseUser::from(user)))
}
