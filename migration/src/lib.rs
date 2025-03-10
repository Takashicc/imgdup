pub use sea_orm_migration::prelude::*;

mod m20250310_123143_reference_images;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250310_123143_reference_images::Migration)]
    }
}
