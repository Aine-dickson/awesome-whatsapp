use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id).not_null())
                    .col(string(User::Username).not_null())
                    .col(string(User::EmailAddress).not_null())
                    .col(string(User::TelContact).not_null())
                    .col(string(User::Verified).not_null().boolean())
                    .col(string(User::CreatedAt).timestamp().not_null())
                    .col(string(User::UpdatedAt).timestamp().not_null())
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

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    EmailAddress,
    TelContact,
    Verified,
    CreatedAt,
    UpdatedAt
}
