use std::{error::Error, fs::OpenOptions, io::Write};

use chrono::Local;

/// Representation of a markdown file
pub struct File<'a> {
    name: &'a str,
    path: &'a str,
    category: Option<&'a str>,
    tags: Option<&'a [String]>,
}

impl<'a> File<'a> {
    pub(crate) fn new(
        name: &'a str,
        path: &'a str,
        category: Option<&'a str>,
        tags: Option<&'a [String]>,
    ) -> Self {
        Self {
            name,
            path,
            category,
            tags,
        }
    }

    /// Creates and writes the Markdown file with the provided meta
    /// data fields
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(self.path)?;

        let mut writer = std::io::BufWriter::new(file);

        // write the metadata for the file
        writer.write_all(self.name.as_bytes())?;
        let divider = self.name.as_bytes().iter().map(|_| "=").collect::<String>();
        writer.write_all(b"\n")?;
        writer.write_all(divider.as_bytes())?;
        writer.write_all(b"\n- Category:")?;
        if let Some(category) = &self.category {
            writer.write_all(category.as_bytes())?;
        }
        writer.write_all(b"\n- Tags:")?;
        if let Some(tags) = self.tags {
            tags.iter().enumerate().try_for_each(|(idx, t)| {
                if idx != 0 {
                    writer.write_all(b", ")?;
                } else {
                    writer.write_all(b" ")?;
                }
                writer.write_all(t.as_bytes())
            })?;
        }
        writer.write_all(b"\n")?;
        let dt = Local::now().format("%d-%b-%Y %H:%M:%S %P %z");
        writer.write_all(format!("- Created: {}", dt).as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }
}
