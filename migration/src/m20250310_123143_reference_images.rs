use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(timestamps(
                Table::create()
                    .table(ReferenceImage::Table)
                    .if_not_exists()
                    .col(pk_auto(ReferenceImage::Id))
                    .col(string(ReferenceImage::Filepath))
                    .col(binary(ReferenceImage::Hash))
                    .to_owned(),
            ))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReferenceImage::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ReferenceImage {
    Table,
    Id,
    Filepath,
    Hash,
}
