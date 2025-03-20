use entity::reference_image::ActiveModel as ReferenceImageActiveModel;
use entity::reference_image::Column as ReferenceImageColumn;
use entity::reference_image::Entity as ReferenceImageEntity;
use entity::reference_image::Model as ReferenceImageModel;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::models;

pub struct ReferenceImageRepository {
    db: DatabaseConnection,
}

impl ReferenceImageRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<ReferenceImageModel>, sea_orm::DbErr> {
        ReferenceImageEntity::find()
            .order_by_asc(ReferenceImageColumn::Id)
            .all(&self.db)
            .await
    }

    pub async fn find_by_filepaths(
        &self,
        filepaths: &Vec<String>,
    ) -> Result<Vec<ReferenceImageModel>, sea_orm::DbErr> {
        ReferenceImageEntity::find()
            .filter(ReferenceImageColumn::Filepath.is_in(filepaths))
            .order_by_asc(ReferenceImageColumn::Id)
            .all(&self.db)
            .await
    }

    pub async fn create_many(
        &self,
        reference_images: Vec<models::reference_image::ReferenceImageInput>,
    ) -> Result<i32, sea_orm::DbErr> {
        let models = reference_images
            .iter()
            .map(|r| ReferenceImageActiveModel {
                filepath: Set(r.filepath.clone()),
                hash: Set(r.hash.clone()),
                ..Default::default()
            })
            .collect::<Vec<ReferenceImageActiveModel>>();

        let result = ReferenceImageEntity::insert_many(models)
            .exec(&self.db)
            .await?;

        Ok(result.last_insert_id)
    }

    pub async fn update(
        &self,
        reference_image: ReferenceImageActiveModel,
    ) -> Result<(), sea_orm::DbErr> {
        ReferenceImageEntity::update(reference_image)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sea_orm::DbErr> {
        let result = ReferenceImageEntity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use migration::MigratorTrait;
    use sea_orm::IntoActiveModel;

    use super::*;

    async fn setup() -> sea_orm::DatabaseConnection {
        let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();

        migration::Migrator::up(&db, None).await.unwrap();

        db
    }

    async fn get_reference_image_repository() -> ReferenceImageRepository {
        let db = setup().await;
        ReferenceImageRepository::new(db)
    }

    #[tokio::test]
    async fn test_find_all() {
        let repo = get_reference_image_repository().await;
        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 0);

        repo.create_many(vec![
            models::reference_image::ReferenceImageInput {
                filepath: "test_1.png".into(),
                hash: "test_hash_1".into(),
            },
            models::reference_image::ReferenceImageInput {
                filepath: "test_2.png".into(),
                hash: "test_hash_2".into(),
            },
        ])
        .await
        .unwrap();

        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].filepath, "test_1.png");
        assert_eq!(result[1].filepath, "test_2.png");
        assert_eq!(result[0].hash, "test_hash_1".as_bytes());
        assert_eq!(result[1].hash, "test_hash_2".as_bytes());
    }

    #[tokio::test]
    async fn test_find_by_filepaths() {
        let repo = get_reference_image_repository().await;

        let target_filepaths = vec![
            "test_1.png".into(),
            "test_2.png".into(),
            "test_3.png".into(),
        ];

        let result = repo.find_by_filepaths(&target_filepaths).await.unwrap();
        assert_eq!(result.len(), 0);

        repo.create_many(vec![
            models::reference_image::ReferenceImageInput {
                filepath: "test_1.png".into(),
                hash: "test_hash_1".into(),
            },
            models::reference_image::ReferenceImageInput {
                filepath: "test_3.png".into(),
                hash: "test_hash_3".into(),
            },
        ])
        .await
        .unwrap();

        let result = repo.find_by_filepaths(&target_filepaths).await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].filepath, "test_1.png");
        assert_eq!(result[1].filepath, "test_3.png");
        assert_eq!(result[0].hash, "test_hash_1".as_bytes());
        assert_eq!(result[1].hash, "test_hash_3".as_bytes());
    }

    #[tokio::test]
    async fn test_create_many() {
        let repo = get_reference_image_repository().await;

        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 0);

        let last_id = repo
            .create_many(vec![
                models::reference_image::ReferenceImageInput {
                    filepath: "test_1.png".into(),
                    hash: "test_hash_1".into(),
                },
                models::reference_image::ReferenceImageInput {
                    filepath: "test_2.png".into(),
                    hash: "test_hash_2".into(),
                },
            ])
            .await
            .unwrap();

        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].filepath, "test_1.png");
        assert_eq!(result[1].filepath, "test_2.png");
        assert_eq!(result[0].hash, "test_hash_1".as_bytes());
        assert_eq!(result[1].hash, "test_hash_2".as_bytes());
        assert_eq!(result[1].id, last_id);
    }

    #[tokio::test]
    async fn test_update() {
        let repo = get_reference_image_repository().await;

        repo.create_many(vec![models::reference_image::ReferenceImageInput {
            filepath: "test_1.png".into(),
            hash: "test_hash_1".into(),
        }])
        .await
        .unwrap();

        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].filepath, "test_1.png");
        assert_eq!(result[0].hash, "test_hash_1".as_bytes());

        let mut update_model = result[0].clone().into_active_model();
        update_model.hash = Set("hash_changed".into());
        repo.update(update_model).await.unwrap();

        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].filepath, "test_1.png");
        assert_eq!(result[0].hash, "hash_changed".as_bytes());
    }

    #[tokio::test]
    async fn test_delete() {
        let repo = get_reference_image_repository().await;

        repo.create_many(vec![models::reference_image::ReferenceImageInput {
            filepath: "test_1.png".into(),
            hash: "test_hash_1".into(),
        }])
        .await
        .unwrap();

        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 1);

        let result = repo.delete(result[0].id).await.unwrap();
        assert_eq!(result, 1);

        let result = repo.find_all().await.unwrap();
        assert_eq!(result.len(), 0);
    }
}
