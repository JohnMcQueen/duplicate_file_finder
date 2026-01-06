use duplicate_finder::gather_files;
use inquire::Text;

fn main() {
    // collecting user input directory to scan
    let query = match gather_user_input() {
        Ok(directory) => directory,
        Err(e) => {
            eprintln!("Error getting user input: {}", e);
            std::process::exit(1);
        }
    };

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

fn gather_user_input() -> std::result::Result<String, inquire::InquireError> {
    Text::new("What directory would you like scanned?").prompt()
}
