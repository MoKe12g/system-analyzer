mod crawler;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _database_url = "sqlite://database.sqlite";
    let root_dir_str = "/home/quantenregen/Schreibtisch/test-bookworm/";
    let excluded_dirs = ["tmp", "home", "proc", "dev", "sys"];

    let database = create_database_connection(_database_url).await?;

    let root_dir = fs::read_dir(root_dir_str).await?;
    let files = crawler::get_files_from_directory(root_dir).await?;

    // filter virtual directories
    let filtered_files = files.iter()
        .filter(|entry|
            {
                !excluded_dirs
                    .contains(&entry.file_name().to_string_lossy().as_ref())
            })
        .collect::<Vec<_>>();


    // crawl the file system for files and directories
    for file in &filtered_files {
        crawler::crawl(file, &database).await?;
    }

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
        .max_connections(25)
        .connect_with(sqlite_options)
        .await?;

    sqlx::migrate!().run(&database).await?;
    Ok(database)
}
