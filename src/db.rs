/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use sea_orm::*;

const DATABASE_URL: &str = "mysql://root:@localhost:3306";
const DB_NAME: &str = "iron_guard";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    Database::connect(format!("{}/{}", DATABASE_URL, DB_NAME)).await
}
