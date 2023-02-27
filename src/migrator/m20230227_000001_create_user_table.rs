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
        "m20230227_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Uuid).uuid().not_null())
                    .col(ColumnDef::new(User::Firstname).string())
                    .col(ColumnDef::new(User::Lastname).string())
                    .col(ColumnDef::new(User::Password).string())
                    .col(ColumnDef::new(User::Meta).text())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().null())
                    .col(ColumnDef::new(User::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Uuid,
    Firstname,
    Lastname,
    Password,
    Meta,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
