use duplicate_finder::gather_files;
use std::env;

fn main() {
    // collecting user input directory to scan
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    println!("User inputed directory to scan: {query}");

    match gather_files(query) {
        Ok(file_map) => {
            file_map.find_true_duplicates().unwrap();
        }
        Err(e) => {
            eprintln!("Error gathering files: {}", e);
        }
    }
}
