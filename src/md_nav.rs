use std::ffi::OsStr;
use std::fs;
use std::io::Result;
use std::path::Path;

use regex::Regex;

pub fn refresh_markdown_navigation(dir: &Path) -> Result<()> {
    let readme_path = dir.join("Readme.md");
    if readme_path.exists() {
        let readme_content = fs::read_to_string(&readme_path)?;

        let parent_link = generate_parent_link(dir);

        let mut links = get_links(dir, &readme_path)?;
        links.sort_by_key(|link| link.to_lowercase());
        let links_section = links.join("\n");

        if !update_links_section(&readme_content, &links_section)? {
            let new_content = generate_new_content(&parent_link, &links_section, &readme_content);
            fs::write(&readme_path, new_content)?;
        }
    }

    update_other_md_files(dir)?;

    Ok(())
}

fn generate_parent_link(dir: &Path) -> String {
    if let Some(parent_dir) = dir.parent() {
        if parent_dir.join("Readme.md").exists() {
            return "[<-](../Readme.md)".to_string();
        }
    }
    String::new()
}

fn update_links_section(readme_content: &str, links_section: &str) -> Result<bool> {
    let link_section_regex = Regex::new(r"(?s)- \[.*?\]\(.*?\)(?:\n- \[.*?\]\(.*?\))*").unwrap();

    if let Some(mat) = link_section_regex.find(readme_content) {
        let existing_links_section = mat.as_str();
        if existing_links_section == links_section {
            return Ok(true);
        }
    }
    Ok(false)
}

fn update_other_md_files(dir: &Path) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            update_single_md_file(&path)?;
        }
    }
    Ok(())
}

fn update_single_md_file(path: &Path) -> Result<()> {
    if let Some(ext) = path.extension() {
        if ext == "md" && path.file_name().unwrap() != "Readme.md" {
            let mut content = fs::read_to_string(path)?;
            if !content.starts_with("[<-](Readme.md)") {
                content = format!("[<-](Readme.md)\n\n{}", content);
                fs::write(path, content)?;
            }
        }
    }
    Ok(())
}

fn get_links(dir: &Path, readme_path: &Path) -> Result<Vec<String>> {
    let mut links = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            add_directory_link(&path, &mut links)?;
        } else {
            add_file_link(&path, readme_path, &mut links)?;
        }
    }
    Ok(links)
}

fn add_directory_link(path: &Path, links: &mut Vec<String>) -> Result<()> {
    let sub_readme = path.join("Readme.md");
    if sub_readme.exists() {
        let dir_name = path.file_name().unwrap().to_str().unwrap();
        let link = format!("<{}/Readme.md>", dir_name);
        links.push(format!("- [{}]({})", dir_name, link));
        refresh_markdown_navigation(path)?;
    }
    Ok(())
}

fn add_file_link(path: &Path, readme_path: &Path, links: &mut Vec<String>) -> Result<()> {
    if let Some(ext) = path.extension() {
        if ext == "md" && path != readme_path {
            let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap_or("");
            let link = if file_stem.contains(' ') {
                format!("<{}.md>", file_stem)
            } else {
                format!("{}.md", file_stem)
            };
            links.push(format!("- [{}]({})", file_stem, link));
        }
    }
    Ok(())
}

fn generate_new_content(parent_link: &str, links_section: &str, readme_content: &str) -> String {
    let link_section_regex = Regex::new(r"(?s)- \[.*?\]\(.*?\)(?:\n- \[.*?\]\(.*?\))*").unwrap();
    if link_section_regex.is_match(readme_content) {
        link_section_regex
            .replace(readme_content, links_section)
            .to_string()
    } else if !links_section.is_empty() {
        format!(
            "{}\n\n{}\n---\n\n{}",
            parent_link, links_section, readme_content
        )
    } else if !readme_content.starts_with(parent_link) {
        format!("{}\n\n{}", parent_link, readme_content)
    } else {
        readme_content.to_string()
    }
}
