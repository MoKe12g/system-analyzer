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

    let pool = SqlitePoolOptions::new()
        .max_connections(80)
        .connect(&database_url).await;
    let database = pool.unwrap();
    // TODO: Write some code
    let root:Vec<DirEntry> = fs::read_dir(root_dir).expect("Failed to read the given directory")
        .filter(|entry| { !["tmp", "home", "proc", "dev", "sys"].contains(&entry.as_ref().unwrap().file_name().to_str().unwrap())}).collect();
    println!("{:?}", (root));
}
