use clap::Parser;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // use clap to parse arguments
    let args = Cli::parse();

    // ?mode=rwc does create the database if it doesn't exist
    // https://stackoverflow.com/questions/72763578/how-to-create-a-sqlite-database-with-rust-sqlx
    let database_url = "sqlite://data.sqlite?mode=rwc";
    let root_dir = "/";
    let excluded_dirs = ["tmp", "home", "proc", "dev", "sys"];

    let database = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.expect("Wasn't able to connect or create the database using the given database_url");

    // Run migrations (create tables, seed data, etc.)
    // 1) cargo install sqlx-cli --no-default-features --features rustls,sqlite
    // 2) sqlx migrate add init
    // 3) put your SQL files under migrations
    let migration_result = sqlx::migrate!().run(&database).await;
    // TODO: Create example data in tables
}
