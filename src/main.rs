use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

#[tokio::main]
async fn main() {
    // ?mode=rwc does create the database if it doesn't exist
    // https://stackoverflow.com/questions/72763578/how-to-create-a-sqlite-database-with-rust-sqlx
    let _database_url = "sqlite://database.sqlite";
    let _root_dir = "/";
    let _excluded_dirs = ["tmp", "home", "proc", "dev", "sys"];

    let database = create_database_connection(_database_url).await;

    // Test insertion as an example for using the database
    let _insertion_result = sqlx::query!("INSERT INTO dpkg_packages (package_name, version, date_installed) VALUES ('system-analyzer', '0.1a', NULL)")
        .execute(&database);
}

async fn create_database_connection(database_url: &str) -> Pool<Sqlite> {
    let database = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(format!("{}{}", &database_url, "?mode=rwc").as_str())
        .await
        .expect("Wasn't able to connect or create the database using the given database_url");

    sqlx::migrate!().run(&database).await.expect("Wasn't able to migrate the database");
    database
}
