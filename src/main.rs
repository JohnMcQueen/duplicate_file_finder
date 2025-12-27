use std::env;
use walkdir::WalkDir;

fn main() {
    println!("Hello, world!");

    // collecting user input directory to scan
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    println!("{query}");

    gather_files(query);
}

fn gather_files(directory: &String) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: figure out if a Result is wanted here. If so, how to return a meaningful error message
    Ok(for entry in WalkDir::new(directory) {
            println!("{}", entry?.path().display());
        })
}
