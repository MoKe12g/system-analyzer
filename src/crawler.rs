use std::io::Error;
use tokio::fs::{DirEntry, ReadDir};

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