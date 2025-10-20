use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::markdown::NotesFrontMatter;

/// Utility function for fetching the yaml front matter of a note
pub fn fetch(reader: &mut BufReader<File>) -> Result<String, Box<dyn Error>> {
    let lines = reader.lines();
    let mut front_matter = String::new();
    let mut in_front_matter = false;
    for line in lines {
        let string = line?;
        if string.as_str().trim() == "---" {
            if in_front_matter {
                break;
            }
            in_front_matter = true;
            continue;
        };
        front_matter.push_str(&string);
        front_matter.push('\n');
    }
    Ok(front_matter)
}

/// Deserialises a str slice into frontmatter
pub fn generate(buf: &str) -> Result<NotesFrontMatter, Box<dyn Error>> {
    Ok(serde_yaml_ng::from_str::<NotesFrontMatter>(buf)?)
}
