use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
enum FileHashError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("WalkDir error: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("Template error: {0}")]
    TemplateError(#[from] indicatif::style::TemplateError),
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    size: u64,
}

fn hash_file_content(file_path: &Path) -> Result<String, FileHashError> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(buffer);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn main() -> Result<(), FileHashError> {
    let mut args = std::env::args();
    let _ = args.next(); // skip the first argument which is the executable name
    let root_path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: file_hasher <root_path>");
            std::process::exit(1);
        }
    };

    let entries: Vec<_> = WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    let pb = ProgressBar::new(entries.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )?
            .progress_chars("#>-"),
    );

    let file_infos: Vec<(String, FileInfo)> = entries
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let result = match hash_file_content(path) {
                Ok(hash) => {
                    let metadata = fs::metadata(path).ok()?;
                    let file_info = FileInfo {
                        path: path.to_path_buf(),
                        size: metadata.len(),
                    };
                    Some((hash, file_info))
                }
                Err(e) => {
                    eprintln!("Failed to hash file {}: {}", path.display(), e);
                    None
                }
            };
            pb.inc(1);
            result
        })
        .collect();

    pb.finish_with_message("Processing complete");

    let mut file_map: HashMap<String, Vec<FileInfo>> = HashMap::new();
    for (hash, file_info) in file_infos {
        file_map
            .entry(hash)
            .or_insert_with(Vec::new)
            .push(file_info);
    }

    let mut file_list: Vec<(&String, &Vec<FileInfo>)> = file_map.iter().collect();
    file_list.sort_by_key(|&(_, files)| -(files.len() as isize));

    for (hash, files) in file_list {
        if let Some(first_file) = files.first() {
            println!("Hash: {}", hash);
            println!("First Instance: {}", first_file.path.display());
            println!("Count: {}", files.len());
            println!("Size: {} bytes\n", first_file.size);
        }
    }

    Ok(())
}
