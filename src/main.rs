use duplicate_finder::FileMap;
use duplicate_finder::gather_files;
use std::env;

fn main() {
    println!("Hello, world!");

    // collecting user input directory to scan
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    println!("{query}");

    let _files = gather_files(query);
}
