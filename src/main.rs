use std::{env, path::Path};

mod md_nav;

/// The main function of the program.
///
/// It takes a command-line argument specifying the path to a markdown file.
/// It then calls the `update_readme_navigation` function from the `md_nav` module
/// to update the navigation in the README file based on the headings in the markdown file.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <markdown_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    if let Err(e) = md_nav::update_readme_navigation(Path::new(file_path)) {
        eprintln!("Error processing file: {}", e);
        std::process::exit(1);
    }
}
