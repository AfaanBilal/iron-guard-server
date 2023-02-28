/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use sea_orm::*;

use crate::Config;

pub(super) async fn connect(config: &Config) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(format!(
        "{}://{}:{}@{}:{}/{}",
        config.db_type,
        config.db_username,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_database
    ));

    opts.sqlx_logging(false);

    Database::connect(opts).await
}
