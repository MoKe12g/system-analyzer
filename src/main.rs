use std::fs;
use std::fs::DirEntry;
use std::ops::Index;
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
    let database_url = "sqlite://data.db?mode=rwc";
    let root_dir = "/";
    let excluded_dirs = ["tmp", "home", "proc", "dev", "sys"];

    let database = SqlitePoolOptions::new()
        .max_connections(80)
        .connect(&database_url).await.expect("Wasn't able to connect or create the database using the given database_url");

    // TODO: Create tables
    // TODO: Crate example data in tables
}
