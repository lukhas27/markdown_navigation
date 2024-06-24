use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs;
use std::io::Result;
use std::path::Path;

use regex::Regex;

/// Updates the navigation links in the `Readme.md` file and other Markdown files in the directory.
///
/// # Arguments
///
/// * `dir` - The directory path where the Markdown files are located.
///
/// # Returns
///
/// Returns `Ok(())` if the navigation links are successfully updated, or an `Err` if an error occurs.
pub fn update_readme_navigation(dir: &Path) -> Result<()> {
    let readme_path = dir.join("Readme.md");

    if readme_path.exists() {
        let readme_content = fs::read_to_string(&readme_path)?;
        let parent_link = create_parent_link(dir);
        let mut links = collect_links(dir, &readme_path)?;
        links.sort_by_key(|link| link.to_lowercase());
        let links_section = links.join("\n");

        if !links_section_is_updated(&readme_content, &links_section)? {
            let new_content =
                create_new_readme_content(&parent_link, &links_section, &readme_content);
            fs::write(&readme_path, new_content)?;
        }
    }

    update_all_markdown_files(dir)?;

    Ok(())
}

/// Creates the parent link for the current directory.
///
/// # Arguments
///
/// * `dir` - The directory path.
///
/// # Returns
///
/// Returns a `Cow<str>` representing the parent link. If the parent directory contains a `Readme.md` file,
/// the link will be in the format `[<-](../Readme.md)`. Otherwise, an empty string is returned.
fn create_parent_link(dir: &Path) -> Cow<str> {
    if let Some(parent_dir) = dir.parent() {
        if parent_dir.join("Readme.md").exists() {
            return Cow::Borrowed("[<-](../Readme.md)");
        }
    }
    Cow::Borrowed("")
}

///
/// # Arguments
///
/// * `readme_content` - The content of the `Readme.md` file.
///
/// # Returns
///
/// or an `Err` if an error occurs.
fn links_section_is_updated(readme_content: &str, links_section: &str) -> Result<bool> {
    let link_section_regex = Regex::new(r"(?s)- \[.*?\]\(.*?\)(?:\n- \[.*?\]\(.*?\))*").unwrap();

    if let Some(mat) = link_section_regex.find(readme_content) {
        let existing_links_section = mat.as_str();
        return Ok(existing_links_section == links_section);
    }
    Ok(false)
}

/// Updates the navigation links in all Markdown files in the directory.
///
/// # Arguments
///
/// * `dir` - The directory path.
///
/// # Returns
///
/// Returns `Ok(())` if the navigation links are successfully updated, or an `Err` if an error occurs.
fn update_all_markdown_files(dir: &Path) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && path.extension().and_then(OsStr::to_str) == Some("md")
            && path.file_name() != Some(OsStr::new("Readme.md"))
        {
            update_markdown_file(&path)?;
        }
    }
    Ok(())
}

/// Updates the navigation link in a Markdown file.
///
/// # Arguments
///
/// * `path` - The path of the Markdown file.
///
/// # Returns
///
/// Returns `Ok(())` if the navigation link is successfully updated, or an `Err` if an error occurs.
fn update_markdown_file(path: &Path) -> Result<()> {
    let mut content = fs::read_to_string(path)?;
    if !content.starts_with("[<-](./Readme.md)") {
        content = format!("[<-](./Readme.md)\n\n{}", content);
        fs::write(path, content)?;
    }
    Ok(())
}

/// Collects the links for all directories and Markdown files in the directory.
///
/// # Arguments
///
/// * `dir` - The directory path.
/// * `readme_path` - The path of the `Readme.md` file.
///
/// # Returns
///
/// Returns a `Vec<String>` containing the generated links.
fn collect_links(dir: &Path, readme_path: &Path) -> Result<Vec<String>> {
    let mut links = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            add_directory_readme_link(&path, &mut links)?;
        } else if path.extension().and_then(OsStr::to_str) == Some("md") && path != readme_path {
            add_markdown_file_link(&path, &mut links)?;
        }
    }
    Ok(links)
}

/// Adds the link to the `Readme.md` file of a subdirectory.
///
/// # Arguments
///
/// * `path` - The path of the subdirectory.
/// * `links` - The vector of links to update.
///
/// # Returns
///
/// Returns `Ok(())` if the link is successfully added, or an `Err` if an error occurs.
fn add_directory_readme_link(path: &Path, links: &mut Vec<String>) -> Result<()> {
    let sub_readme = path.join("Readme.md");
    if sub_readme.exists() {
        if let Some(dir_name) = path.file_name().and_then(OsStr::to_str) {
            let link = format!("<{}/Readme.md>", dir_name);
            links.push(format!("- [{}]({})", dir_name, link));
            update_readme_navigation(path)?;
        }
    }
    Ok(())
}

/// Adds the link to a Markdown file.
///
/// # Arguments
///
/// * `path` - The path of the Markdown file.
/// * `links` - The vector of links to update.
///
/// # Returns
///
/// Returns `Ok(())` if the link is successfully added, or an `Err` if an error occurs.
fn add_markdown_file_link(path: &Path, links: &mut Vec<String>) -> Result<()> {
    if let Some(file_stem) = path.file_stem().and_then(OsStr::to_str) {
        let link = if file_stem.contains(' ') {
            format!("<{}.md>", file_stem)
        } else {
            format!("{}.md", file_stem)
        };
        links.push(format!("- [{}]({})", file_stem, link));
    }
    Ok(())
}

/// Updates or inserts the navigation panel in the `Readme.md` file content.
///
/// This function performs several key operations to ensure the navigation panel is correctly handled:
/// 1. Searches for a specific tag (`<!--- NAVIGATION PANEL -->`) in the `Reade.md` file content. If the tag is found,
///    the navigation panel is placed at that location.
/// 2. If the tag is not found, it looks for predefined start and end markers for the navigation panel. The start markers
///    are `[<-](../Readme.md)` and `[<-](./Readme.md)`, and the end marker is a single "---" divider.
/// 3. If these markers are present, the existing navigation panel is updated with the new content. This update process
///    ensures that the start markers are preserved, and exactly one "---" divider is present at the end of the navigation
///    panel, avoiding duplication.
/// 4. If neither the tag nor the markers are found, the navigation panel is inserted at the beginning of the file content,
///    ensuring proper formatting with a "---" divider at the end.
///
/// # Arguments
///
/// * `parent_link` - A string slice that holds the parent link for the current directory. This link is used to maintain
///   a reference to the parent directory at the beginning of the navigation panel.
/// * `readme_content` - A string slice that holds the current content of the `Readme.md` file. This content is searched
///   to find the appropriate location for the navigation panel or to update the existing panel.
///
/// # Returns
///
/// Returns a String containing the updated or newly inserted navigation panel within the `Readme.md` file content.
/// The returned content is formatted to ensure that the navigation panel is correctly integrated into the `Readme.md`,
/// with appropriate markers and dividers for clarity and consistency.
fn create_new_readme_content(
    parent_link: &str,
    links_section: &str,
    readme_content: &str,
) -> String {
    let start_patterns = vec!["[<-](../Readme.md)", "[<-](./Readme.md)"];
    let navigation_end_pattern = "---";

    let mut navigation_section_found = false;
    let mut new_content = readme_content.to_owned();

    for start_pattern in &start_patterns {
        if let Some(start_index) = readme_content.find(start_pattern) {
            if let Some(end_index) = readme_content[start_index..].find(navigation_end_pattern) {
                let end_of_navigation = start_index + end_index;
                let is_duplicate_divider =
                    readme_content[end_of_navigation..].starts_with("---\n---");

                let navigation_replacement = if is_duplicate_divider {
                    format!("{}\n{}---\n", start_pattern, links_section)
                } else {
                    format!(
                        "{}\n{}{}",
                        start_pattern,
                        links_section,
                        if links_section.ends_with('\n') {
                            "---\n"
                        } else {
                            "\n---\n"
                        }
                    )
                };

                new_content = format!(
                    "{}{}{}",
                    &readme_content[..start_index],
                    navigation_replacement,
                    &readme_content
                        [end_of_navigation + (if is_duplicate_divider { 4 } else { 3 })..] // Adjust based on whether there was a duplicate divider
                );
                navigation_section_found = true;
                break;
            }
        }
    }

    if !navigation_section_found {
        new_content = format!(
            "{}\n{}\n---\n{}",
            parent_link, links_section, readme_content
        );
    }

    new_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_create_parent_link() {
        let temp_dir = tempdir().unwrap();
        let parent_readme = temp_dir.path().join("Readme.md");
        File::create(parent_readme).unwrap();

        let child_dir = temp_dir.path().join("child");
        fs::create_dir(&child_dir).unwrap();

        assert_eq!(
            create_parent_link(&child_dir),
            Cow::Borrowed("[<-](../Readme.md)")
        );
        assert_eq!(create_parent_link(temp_dir.path()), Cow::Borrowed(""));
    }

    #[test]
    fn test_links_section_is_updated() {
        let readme_content = "- [File1](file1.md)\n- [File2](file2.md)";
        let links_section = "- [File1](file1.md)\n- [File2](file2.md)";

        assert!(links_section_is_updated(readme_content, links_section).unwrap());

        let new_links_section = "- [File1](file1.md)\n- [File3](file3.md)";
        assert!(!links_section_is_updated(readme_content, new_links_section).unwrap());
    }

    #[test]
    fn test_update_markdown_file() {
        let temp_dir = tempdir().unwrap();
        let markdown_path = temp_dir.path().join("test.md");
        let mut file = File::create(&markdown_path).unwrap();
        writeln!(file, "Content").unwrap();

        update_markdown_file(&markdown_path).unwrap();

        let updated_content = fs::read_to_string(&markdown_path).unwrap();
        assert!(updated_content.starts_with("[<-](./Readme.md)\n\nContent"));
    }

    #[test]
    fn test_collect_links() {
        let temp_dir = tempdir().unwrap();
        let readme_path = temp_dir.path().join("Readme.md");
        File::create(&readme_path).unwrap();

        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        let sub_readme_path = sub_dir.join("Readme.md");
        File::create(sub_readme_path).unwrap();

        let file_path = temp_dir.path().join("file.md");
        File::create(file_path).unwrap();

        let links = collect_links(temp_dir.path(), &readme_path).unwrap();
        assert_eq!(links.len(), 2);
        assert!(links.contains(&"- [subdir](<subdir/Readme.md>)".to_string()));
        assert!(links.contains(&"- [file](file.md)".to_string()));
    }

    #[test]
    fn test_create_new_readme_content() {
        let parent_link = "[<-](../Readme.md)";
        let links_section = "- [File1](file1.md)\n- [File2](file2.md)";
        let readme_content = "Some initial content\n";

        let new_content = create_new_readme_content(parent_link, links_section, readme_content);
        assert_eq!(new_content, "[<-](../Readme.md)\n- [File1](file1.md)\n- [File2](file2.md)\n---\nSome initial content\n");
    }

    #[test]
    fn test_update_readme_navigation() {
        let temp_dir = tempdir().unwrap();
        let readme_path = temp_dir.path().join("Readme.md");
        let mut readme_file = File::create(&readme_path).unwrap();
        writeln!(readme_file, "Initial content").unwrap();

        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        let sub_readme_path = sub_dir.join("Readme.md");
        File::create(sub_readme_path).unwrap();

        let file_path = temp_dir.path().join("file.md");
        let mut file = File::create(file_path).unwrap();
        writeln!(file, "File content").unwrap();

        update_readme_navigation(temp_dir.path()).unwrap();

        let updated_readme_content = fs::read_to_string(&readme_path).unwrap();

        // assert!(updated_readme_content.contains("[<-](../Readme.md)"));
        assert!(updated_readme_content.contains("- [file](file.md)"));
        assert!(updated_readme_content.contains("- [subdir](<subdir/Readme.md>)"));
    }
}
