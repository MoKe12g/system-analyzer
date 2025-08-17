use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let _database_url = "sqlite://database.sqlite";
    let _root_dir = "/";
    let _excluded_dirs = ["tmp", "home", "proc", "dev", "sys"];

    let database = create_database_connection(_database_url).await?;

    // Test insertion as an example for using the database
    sqlx::query!("INSERT OR IGNORE INTO dpkg_packages (package_name, version, date_installed) VALUES (?, ?, CURRENT_TIMESTAMP)",
    "system-analyzer", "0.1a")
        .execute(&database).await?;
    Ok(())
}

async fn create_database_connection(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    let sqlite_options = SqliteConnectOptions::from_str(database_url)?
        .foreign_keys(true)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let database = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(sqlite_options)
        .await?;

    sqlx::migrate!().run(&database).await?;
    Ok(database)
}
