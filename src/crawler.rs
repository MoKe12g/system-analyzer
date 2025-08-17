use async_recursion::async_recursion;
use log::debug;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Sqlite;
use std::fs::{FileType, Metadata};
use std::os::unix::fs::MetadataExt;
use tokio::fs;
use tokio::fs::{DirEntry, ReadDir};

pub struct File
{
    path: String,
    size: u32,
    is_folder: bool,
    package: String,
    is_changed: bool,
}

impl File {
    async fn from_read_dir(input_file: &DirEntry, input_file_type: &FileType, input_file_metadata: &Metadata) -> Self {
        File {
            path: input_file.path().display().to_string(),
            size: input_file_metadata.size() as u32,
            is_folder: input_file_type.is_dir(),
            package: "".to_string(),
            is_changed: true,
        }
    }
}

impl File {
    pub fn new(path: String, size: u64, is_folder: bool, package: String, is_changed: bool) -> Self {
        File { path, size: size as u32, is_folder, package, is_changed }
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_size(&self) -> &u32 {
        &self.size
    }

    pub fn is_folder(&self) -> &bool {
        &self.is_folder
    }

    pub fn get_package(&self) -> &String {
        &self.package
    }

    pub fn is_changed(&self) -> &bool {
        &self.is_changed
    }
}

pub async fn get_files_from_directory(mut dir: ReadDir) -> Result<Vec<DirEntry>, Error> {
    // get files in directory
    let mut files: Vec<DirEntry> = Vec::new();
    {
        let mut eof: bool = false;
        while !eof {
            let file = dir.next_entry().await?;
            match file {
                Some(f) => files.push(f),
                None => eof = true,
            }
        }
    }
    Ok(files)
}

#[async_recursion]
pub(crate) async fn crawl(input_file: &DirEntry, database: &Pool<Sqlite>) -> Result<(), Error> {
    let file_type = input_file.file_type().await?;
    let file_metadata = input_file.metadata().await?;
    // create entry in database
    let file = File::from_read_dir(input_file, &file_type, &file_metadata).await;
    sqlx::query!("INSERT INTO files (path, size, is_folder, package, is_changed) VALUES (?, ?, ?, NULL, NULL);",
    file.path, file.size, file.is_folder)
        .execute(database).await?; // TODO: Is it wise to dereference here?
    debug!("Added {} to the database", file.path);
    // TODO: Improve performance using https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html
    // continue crawl
    if file_type.is_dir() {
        match fs::read_dir(input_file.path()).await {
            Ok(folder) => {
                let files = get_files_from_directory(folder).await;
                match files {
                    Ok(files) => {
                        for file in &files {
                            crawl(file, database).await?;
                        }
                    },
                    Err(err) => { println!("Skipped files in Folder {} because error occurred. {:?}", file.path, err); }
                }
            },
            Err(err) => { println!("Skiepped {} because error occurred. {:?}", file.path, err); }
        }
    }
    Ok(())
}