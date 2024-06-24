use std::{env, path::Path};

mod md_nav;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for help flag
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("mdnav - A tool for navigating through markdown files\n");
        println!("Usage:");
        println!("  mdnav <folder_path>      Process the specifed folder and add navigation links to all markdown files");
        println!("  mdnav --version | -v     Print version information");
        println!("  mdnav --help | -h        Print this help message");
        std::process::exit(0);
    }

    // Check for version flag
    if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
        println!("mdnav version: {}", VERSION);
        std::process::exit(0);
    }

    // Existing logic to process markdown file
    if args.len() != 2 {
        eprintln!("Usage: mdnav <markdown_file>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    if let Err(e) = md_nav::update_readme_navigation(Path::new(file_path)) {
        eprintln!("Error processing file: {}", e);
        std::process::exit(1);
    }
}
