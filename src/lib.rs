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

    pub fn get_duplicates(&self) -> HashMap<String, Vec<String>> {
        self.files
            .iter()
            .filter(|(_, files)| files.len() > 1)
            .map(|(size, files)| (size.clone(), files.clone()))
            .collect()
    }

    pub fn print_duplicates(&self) {
        for (size, files) in &self.get_duplicates() {
            if files.len() > 1 {
                println!("\nFiles with size {} bytes:", size);
                for file in files {
                    println!("  - {}", file);
                }
            }
        }
    }

    pub fn find_true_duplicates(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (_, files) in &self.files {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_file_map() {
        let file_map = FileMap::default();

        assert_eq!(file_map.files.len(), 0);
    }

    #[test]
    fn test_adding_file() {
        let file_name = "test".to_string();
        let file_length: u64 = 883223;
        let mut file_map = FileMap::default();

        file_map.add_file(file_length, file_name);

        assert_eq!(
            file_map.files.get("883223").unwrap(),
            &vec!["test".to_string()]
        );
    }

    #[test]
    fn test_get_duplicates() {
        let mut file_map = FileMap::default();
        file_map.add_file(100, "file1.txt".to_string());
        file_map.add_file(100, "file2.txt".to_string());
        file_map.add_file(200, "file3.txt".to_string());

        let duplicates = file_map.get_duplicates();

        assert_eq!(duplicates.len(), 1);
        assert!(duplicates.contains_key("100"));
        assert_eq!(duplicates.get("100").unwrap().len(), 2);
    }
}
