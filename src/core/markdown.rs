use std::{error::Error, fs::OpenOptions, io::Write};

use chrono::Local;
use serde::{Deserialize, Serialize};

/// Representation of a markdown file
pub struct File<'a> {
    name: &'a str,
    path: &'a str,
    category: Option<&'a str>,
    tags: Option<Vec<String>>,
}

/// Representation of the yaml metadata field
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Metadata<'a> {
    pub(crate) category: Option<&'a str>,
    pub(crate) subcategories: Option<Vec<String>>,
    pub(crate) hidden: bool,
}

impl<'a> Metadata<'a> {
    fn new(category: Option<&'a str>) -> Self {
        Self {
            category: category.map(|s| s.split("/").take(1).next().unwrap()),
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
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct NotesFrontMatter<'a> {
    pub(crate) title: &'a str,
    pub(crate) date: String,
    pub(crate) tags: Option<Vec<String>>,
    #[serde(borrow)]
    // notes specific metadata
    pub(crate) notes_metadata: Metadata<'a>,
}

impl<'a> NotesFrontMatter<'a> {
    fn new(
        title: &'a str,
        category: Option<&'a str>,
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
        let dt = Local::now().format("%Y-%m-%dT%H:%M:%S%:z");
        let title = self.name.replace("-", " ");
        let value =
            NotesFrontMatter::new(title.as_str(), self.category, self.tags, format!("{}", dt));

        let metadata = serde_yaml_ng::to_string(&value)?;
        writer.write_all(b"---\n")?;
        writer.write_all(metadata.as_bytes())?;
        writer.write_all(b"---\n\n")?;

        writer.write_all(self.name.as_bytes())?;
        let divider = self.name.as_bytes().iter().map(|_| "=").collect::<String>();
        writer.write_all(b"\n")?;
        writer.write_all(divider.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}
