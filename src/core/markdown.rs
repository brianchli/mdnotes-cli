use std::{error::Error, fs::OpenOptions, io::Write};

use chrono::Local;
use serde::{Deserialize, Serialize};

/// Representation of a markdown file
pub struct File<'a> {
    name: &'a str,
    path: &'a str,
    category: Option<&'a str>,
    tags: Option<&'a [&'a str]>,
}

/// Representation of the yaml metadata field
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Metadata<'a> {
    pub(crate) category: Option<&'a str>,
    pub(crate) tags: Option<Vec<&'a str>>,
    pub(crate) created: String,
    pub(crate) hidden: bool,
}

impl<'a> Metadata<'a> {
    fn new(category: Option<&'a str>, tags: Option<Vec<&'a str>>, created: String) -> Self {
        Self {
            category,
            tags,
            created,
            hidden: false,
        }
    }
}

/// Representation of the front matter at the top of each note
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct NotesFrontMatter<'a> {
    #[serde(borrow)]
    pub(crate) metadata: Metadata<'a>,
}

impl<'a> NotesFrontMatter<'a> {
    fn new(category: Option<&'a str>, tags: Option<Vec<&'a str>>, created: String) -> Self {
        Self {
            metadata: Metadata::new(category, tags, created),
        }
    }
}

impl<'a> File<'a> {
    pub(crate) fn new(
        name: &'a str,
        path: &'a str,
        category: Option<&'a str>,
        tags: Option<&'a [&'a str]>,
    ) -> Self {
        Self {
            name,
            path,
            category,
            tags,
        }
    }

    /// Creates and writes the Markdown file with the provided metadata fields
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(self.path)?;

        let mut writer = std::io::BufWriter::new(file);
        let dt = Local::now().format("%d-%b-%Y %H:%M:%S %P %z");
        let value = NotesFrontMatter::new(
            self.category,
            self.tags.map(|t| t.to_owned()),
            format!("{}", dt),
        );

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
