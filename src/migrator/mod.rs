/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use sea_orm_migration::prelude::*;

mod m20230227_000001_create_user_table;
mod m20230227_000002_create_category_table;
mod m20230227_000003_create_item_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230227_000001_create_user_table::Migration),
            Box::new(m20230227_000002_create_category_table::Migration),
            Box::new(m20230227_000003_create_item_table::Migration),
        ]
    }
}
