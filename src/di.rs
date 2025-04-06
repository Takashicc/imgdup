use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

use crate::repositories::reference_image_repository::ReferenceImageRepository;

pub struct Container {
    pub reference_image_repository: Arc<ReferenceImageRepository>,
}

impl Container {
    pub async fn new(db: &DatabaseConnection) -> Self {
        let reference_image_repository = Arc::new(ReferenceImageRepository::new(db.clone()));

        Self {
            reference_image_repository,
        }
    }
}

static CONTAINER: OnceCell<Container> = OnceCell::const_new();

pub async fn get_container() -> &'static Container {
    CONTAINER
        .get_or_init(|| async {
            let db = crate::adapter::sqlite::get_db().await;
            Container::new(db).await
        })
        .await
}
