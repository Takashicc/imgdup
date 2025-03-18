use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{sync::Arc, time::Duration};
use tokio::sync::OnceCell;

const DATABASE_URL: &str = "sqlite://db.sqlite?mode=rwc";

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async { establish_connection().await.unwrap() })
        .await
}

async fn establish_connection() -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(DATABASE_URL);
    opt.connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(5))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt)
        .await
        .with_context(|| "Failed to create database connection")?;

    db.ping().await.with_context(|| "Failed to ping database")?;

    Ok(db)
}
