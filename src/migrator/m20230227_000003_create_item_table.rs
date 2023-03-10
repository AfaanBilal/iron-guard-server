/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard-server
 */
use sea_orm_migration::prelude::*;

use super::{
    m20230227_000001_create_user_table::User, m20230227_000002_create_category_table::Category,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230227_000003_create_item_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Item::Table)
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::Uuid).string().unique_key().not_null())
                    .col(ColumnDef::new(Item::CategoryId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-category_id")
                            .from(Item::Table, Item::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .col(ColumnDef::new(Item::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-user_id")
                            .from(Item::Table, Item::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::Description).text())
                    .col(
                        ColumnDef::new(Item::Quantity)
                            .integer()
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Item::Meta).text())
                    .col(
                        ColumnDef::new(Item::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Item::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(ColumnDef::new(Item::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Item {
    Table,
    Id,
    Uuid,
    CategoryId,
    UserId,
    Name,
    Description,
    Quantity,
    Meta,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
