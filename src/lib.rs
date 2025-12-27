use std::collections::HashMap;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileMap {
    // keys will be file sizes and value will be vectors of file paths
    pub files: HashMap<String, Vec<String>>,
}

impl FileMap {
    pub fn new() -> FileMap {
        FileMap {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file_size: u64, file_name: String) {
        self.files
            .entry(file_size.to_string())
            .or_insert(Vec::new())
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
}
pub fn gather_files(directory: &String) -> Result<(), Box<dyn std::error::Error>> {
    // let mut file_paths = Vec::new();
    let mut file_map = FileMap::new();

    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let file_size = metadata.len();
            let file_path = entry.path().to_path_buf();

            println!("{} - {} bytes", file_path.display(), file_size);

            file_map.add_file(file_size, file_path.display().to_string());
        }
    }

    println!("{:#?}", file_map);

    Ok(())
}
