use std::{error::Error, fs::OpenOptions, io::Write};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Representation of a markdown file
pub struct File<'a> {
    name: &'a str,
    path: &'a str,
    category: Option<&'a str>,
    tags: Option<Vec<String>>,
}

/// Representation of the yaml metadata field
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub(crate) struct Metadata {
    pub(crate) category: Option<String>,
    pub(crate) subcategories: Option<Vec<String>>,
    pub(crate) hidden: bool,
}

impl Metadata {
    fn new(category: Option<&str>) -> Self {
        Self {
            category: category.map(|s| s.split("/").take(1).next().unwrap().into()),
            subcategories: category.map(|s| {
                s.split("/")
                    .skip(1)
                    .map(|s| s.to_owned())
                    .filter(|s| !s.is_empty())
                    .collect()
            }),
            hidden: false,
        }
    }
}

/// Representation of the front matter at the top of each note
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub(crate) struct NotesFrontMatter {
    pub(crate) title: String,
    pub(crate) date: String,
    pub(crate) tags: Option<Vec<String>>,
    // notes specific metadata
    pub(crate) notes_metadata: Metadata,
}

impl NotesFrontMatter {
    pub fn new(
        title: String,
        category: Option<&str>,
        tags: Option<Vec<String>>,
        date: String,
    ) -> Self {
        Self {
            title,
            tags,
            date,
            notes_metadata: Metadata::new(category),
        }
    }
}

impl<'a> File<'a> {
    pub(crate) fn new(
        name: &'a str,
        path: &'a str,
        category: Option<&'a str>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            name,
            path,
            category,
            tags,
        }
    }

    /// Creates and writes the Markdown file with the provided metadata fields
    pub fn write(self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(self.path)?;

        let mut writer = std::io::BufWriter::new(file);
        let title = if DateTime::parse_from_rfc3339(self.name).is_ok() {
            self.name.to_string()
        } else {
            self.name.replace("-", " ")
        };
        let frontmatter =
            NotesFrontMatter::new(title, self.category, self.tags, Local::now().to_rfc3339());
        let frontmatter_str = serde_yaml_ng::to_string(&frontmatter)?;
        writer.write_all(b"---\n")?;
        writer.write_all(frontmatter_str.as_bytes())?;
        writer.write_all(b"---\n\n")?;
        writer.flush()?;
        Ok(())
    }
}
