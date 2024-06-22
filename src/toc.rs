use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;


pub fn toc_exists(content: &str) -> bool {
    let re = Regex::new(r"(?msi)^# Table of Contents.*").unwrap();
    re.is_match(content)
}

pub fn split_toc_and_content(makdown: &str) -> (&str, &str) {
    let re = Regex::new(r"(?msi)^# Table of makdowns.*").unwrap();
    let old_toc_end = re.find(makdown).map_or(0, |m| m.end());
    let next_header_index = makdown[old_toc_end..].find("\n# ").unwrap_or_else(|| makdown.len() - old_toc_end) + old_toc_end;
    makdown.split_at(next_header_index)
}

pub fn update_toc(old_toc: &str, new_toc: &str, content: &str) -> String {
    if new_toc != old_toc {
        format!("{}\n\n{}", new_toc, content)
    } else {
        format!("{}\n\n{}", old_toc, content)
    }
}

pub fn generate_toc_and_add_anchors(content: &str) -> (String, String) {
    let parser = Parser::new_ext(content, Options::empty());
    let mut toc = String::new();
    let mut new_content = String::new();
    let mut current_level: Option<HeadingLevel> = None;
    let mut in_heading = false;
    let mut current_header_text = String::new();

    toc.push_str("## Table of Contents\n\n");

    for event in parser {
        match event {
            Event::Start(Tag::Heading{level, ..}) => {
                current_level = Some(level);
                in_heading = true;
                new_content.push_str(&format!("\n{}", "#".repeat(level as usize)));
                new_content.push(' ');
                current_header_text.clear();
            }
            Event::Text(text) => {
                if let Some(_level) = current_level {
                    if in_heading {
                        let anchor = format!("<a id=\"{}\"></a>\n", text.to_lowercase().replace(" ", "-"));
                        if !text.contains(&anchor) {
                            new_content.push_str(&format!("{} {}", text, anchor));
                        } else {
                            new_content.push_str(&text);
                        }
                        current_header_text.push_str(&text);
                        in_heading = false;
                    } else {
                        new_content.push_str(&text);
                    }
                } else {
                    new_content.push_str(&text);
                }
            }
            Event::End(..) => {
                if let Some(current_level) = current_level {
                    let level_number = match current_level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    if current_header_text.to_lowercase() != "table of contents" {
                        let header_text = current_header_text.clone();
                        let anchor = header_text.to_lowercase().replace(' ', "-");
                        toc.push_str(&format!(
                            "{}- [{}](#{})\n",
                            "  ".repeat((level_number - 1) as usize),
                            header_text,
                            anchor
                        ));
                    }
                }
                current_level = None;
                new_content.push('\n');
            }
            _ => {}
        }
    }

    (toc, new_content)
}
                
pub fn process_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let markdown = fs::read_to_string(file_path)?;
    // println!(fromat!(!toc_exists(&markdown));
    if !toc_exists(&markdown) {
        let (new_toc, new_content) = generate_toc_and_add_anchors(&markdown);
        let new_markdown = format!("{}\n\n{}", new_toc, new_content);
        let mut file = File::create("README_TOCS.md")?;
        file.write_all(new_markdown.as_bytes())?;   
    } else {
        let (old_toc, content) = split_toc_and_content(&markdown);
        let (new_toc, new_content) = generate_toc_and_add_anchors(content);
        if (old_toc != new_toc) || (content != new_content) {
            let new_markdown = update_toc(old_toc, &new_toc, &new_content);
            let mut file = File::create("README_TOCS.md")?;
            file.write_all(new_markdown.as_bytes())?;
        }
    }
    Ok(())
}
