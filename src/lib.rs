use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct FileMap {
    // keys will be file sizes and value will be vectors of file paths
    pub files: HashMap<String, Vec<String>>,
}

impl FileMap {
    pub fn add_file(&mut self, file_size: u64, file_name: String) {
        self.files
            .entry(file_size.to_string())
            .or_default()
            .push(file_name);
    }

    pub fn print_duplicates(&self) {
        for (size, files) in &self.files {
            if files.len() > 1 {
                println!("\nFiles with size {} bytes:", size);
                for file in files {
                    println!("  - {}", file);
                }
            }
        }
    }

    pub fn find_true_duplicates(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (_size, files) in &self.files {
            // Only hash if there are duplicates potentially
            if files.len() > 1 {
                let mut hashed_files: HashMap<String, Vec<String>> = HashMap::new();

                for file_path in files {
                    let hash = hash_files(file_path)?;
                    hashed_files
                        .entry(hash)
                        .or_default()
                        .push(file_path.clone());
                }

                // TODO: printing for now but in future should do something with the duplicates
                for (hash, paths) in &hashed_files {
                    if paths.len() > 1 {
                        println!(" Duplicates found (hash: {})", hash);
                        for path in paths {
                            println!("    - {}", path);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn check_for_duplicates(directory: &String) {
    let file_map = gather_files(directory);

    match file_map {
        Ok(v) => println!("result {:#?}", v),
        Err(e) => panic!("error!: {e}"),
    }
}

pub fn gather_files(directory: &String) -> Result<FileMap, Box<dyn std::error::Error>> {
    let mut file_map = FileMap::default();

    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let file_size = metadata.len();
            let file_path = entry.path().to_path_buf();

            println!("{}", file_path.display());

            file_map.add_file(file_size, file_path.display().to_string());
        }
    }

    Ok(file_map)
}

fn hash_files(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;

    Ok(format!("{:x}", hasher.finalize()))
}
