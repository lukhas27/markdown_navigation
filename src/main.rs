use std::{env, path::Path};

mod md_nav;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <markdown_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    if let Err(e) = md_nav::refresh_markdown_navigation(Path::new(file_path)) {
        eprintln!("Error processing file: {}", e);
        std::process::exit(1);
    }
}
