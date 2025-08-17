use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let _database_url = "sqlite://database.sqlite";
    let _root_dir = "/";
    let _excluded_dirs = ["tmp", "home", "proc", "dev", "sys"];

    let database = create_database_connection(_database_url).await.expect("Failed to create database connection");

    // Test insertion as an example for using the database
    sqlx::query!("INSERT INTO dpkg_packages (package_name, version, date_installed) VALUES ('system-analyzer', '0.1a', NULL)")
        .execute(&database).await.expect("Couldn't insert into database");
}

async fn create_database_connection(database_url: &str) -> Option<Pool<Sqlite>> {
    let sqlite_options = SqliteConnectOptions::from_str(database_url)
        .expect("Failed to parse database url")
        .foreign_keys(true)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let database = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(sqlite_options)
        .await
        .expect("Wasn't able to connect or create the database using the given database_url");

    sqlx::migrate!().run(&database).await.expect("Wasn't able to migrate the database");
    Some(database)
}
