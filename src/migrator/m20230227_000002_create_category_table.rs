/**
 * Iron Guard Server
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/iron-guard
 */
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230227_000002_create_category_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Category::Table)
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::Uuid).string().unique_key().not_null())
                    .col(ColumnDef::new(Category::ParentId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category-parent_id")
                            .from(Category::Table, Category::ParentId)
                            .to(Category::Table, Category::Id),
                    )
                    .col(ColumnDef::new(Category::Name).string())
                    .col(ColumnDef::new(Category::Description).text())
                    .col(ColumnDef::new(Category::Meta).text())
                    .col(ColumnDef::new(Category::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(Category::UpdatedAt).timestamp().null())
                    .col(ColumnDef::new(Category::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Category {
    Table,
    Id,
    Uuid,
    ParentId,
    Name,
    Description,
    Meta,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
