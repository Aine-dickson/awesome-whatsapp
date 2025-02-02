use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Verification::Table)
                    .if_not_exists()
                    .col(pk_uuid(Verification::Id))
                    .col(string(Verification::EmailAddress).not_null())
                    .col(string(Verification::Token).big_integer().not_null())
                    .col(string(Verification::Expiry).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Verification::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Verification {
    Table,
    Id,
    EmailAddress,
    Token,
    Expiry
}
