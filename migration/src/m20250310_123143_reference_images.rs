use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReferenceImage::Table)
                    .if_not_exists()
                    .col(pk_auto(ReferenceImage::Id))
                    .col(string(ReferenceImage::Filepath))
                    .col(binary(ReferenceImage::Hash))
                    .col(timestamp(ReferenceImage::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(ReferenceImage::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(sea_orm::Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TRIGGER IF NOT EXISTS reference_image_updated_at
                AFTER UPDATE ON reference_image
                FOR EACH ROW
                BEGIN
                    UPDATE reference_image
                    SET updated_at = CURRENT_TIMESTAMP
                    WHERE id = NEW.id;
                END
                "#,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(sea_orm::Statement::from_string(
                manager.get_database_backend(),
                "DROP TRIGGER IF EXISTS reference_image_updated_at",
            ))
            .await?;

        manager
            .drop_table(Table::drop().table(ReferenceImage::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ReferenceImage {
    Table,
    Id,
    Filepath,
    Hash,
    CreatedAt,
    UpdatedAt,
}
