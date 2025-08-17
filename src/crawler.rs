use async_recursion::async_recursion;
use log::{debug, warn};
use sqlx::Error;
use sqlx::Pool;
use sqlx::Sqlite;
use std::fs::{FileType, Metadata};
use tokio::fs;
use tokio::fs::{DirEntry, ReadDir};

#[derive(Debug, Clone)]
pub struct File
{
    path: String,
    size: i64,
    is_folder: bool,
    package: Option<String>,
    is_changed: Option<bool>,
}

impl File {
    fn from_read_dir(input_file: &DirEntry, input_file_type: &FileType, input_file_metadata: &Metadata) -> Self {
        File {
            path: input_file.path().display().to_string(),
            size: input_file_metadata.len() as i64,
            is_folder: input_file_type.is_dir(),
            package: None,
            is_changed: None,
        }
    }

    pub fn new(path: String, size: i64, is_folder: bool, package: Option<String>, is_changed: Option<bool>) -> Self {
        File { path, size, is_folder, package, is_changed }
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_size(&self) -> i64 {
        self.size
    }

    pub fn is_folder(&self) -> bool {
        self.is_folder
    }

    pub fn get_package(&self) -> &Option<String> {
        &self.package
    }

    pub fn is_changed(&self) -> &Option<bool> {
        &self.is_changed
    }
}

pub async fn get_files_from_directory(mut dir: ReadDir) -> std::io::Result<Vec<DirEntry>> {
    // get files in directory
    let mut files: Vec<DirEntry> = Vec::new();
    while let Some(entry) = dir.next_entry().await? {
        files.push(entry);
    }
    Ok(files)
}

#[async_recursion]
pub(crate) async fn crawl(input_file: &DirEntry, database: &Pool<Sqlite>) -> Result<(), Error> {
    let file_type = input_file.file_type().await?;
    let file_metadata = input_file.metadata().await?;
    // create entry in database
    let file = File::from_read_dir(input_file, &file_type, &file_metadata);
    sqlx::query!("INSERT INTO files (path, size, is_folder, package, is_changed) VALUES (?, ?, ?, ?, ?);",
    file.path, file.size, file.is_folder, file.package, file.is_changed)
        .execute(database).await?;
    debug!("Added {} to the database", file.path);
    // TODO: Improve performance using https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html
    // continue crawl
    if file_type.is_dir() && !file_type.is_symlink() {
        match fs::read_dir(input_file.path()).await {
            Ok(folder) => {
                let files = get_files_from_directory(folder).await;
                match files {
                    Ok(files) => {
                        for file in &files {
                            crawl(file, database).await?;
                        }
                    },
                    Err(err) => { warn!("Skipped files in Folder {} because error occurred. {:?}", file.path, err); }
                }
            },
            Err(err) => { warn!("Skipped {} because error occurred. {:?}", file.path, err); }
        }
    }
    Ok(())
}