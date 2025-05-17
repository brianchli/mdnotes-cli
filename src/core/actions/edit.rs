use std::{
    error::Error,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use clap::ArgMatches;

use crate::core::markdown;

use super::{Command, Configuration};

/// Representation of an edit command and the context
/// needed for an edit command
pub struct Edit<'a> {
    name: &'a str,
    path: PathBuf,
    category: Option<&'a str>,
    tags: Option<Vec<&'a str>>,
}

impl<'a> Command<'a> for Edit<'a> {
    fn new(args: &'a ArgMatches, conf: &Configuration) -> Result<Self, Box<dyn Error>> {
        // We always demand an argument name
        let name = args.get_one::<String>("name").unwrap();
        let category = args.get_one::<String>("category").map(|s| s.as_str());
        let mut path = if let Some(category) = category {
            PathBuf::from(format!(
                "{}/{}",
                conf.settings.path,
                validate_path(Path::new(category))?
            ))
        } else {
            PathBuf::from(&conf.settings.path)
        };

        path.push(format!("{name}.md"));

        Ok(Self {
            name: validate_name(name)?,
            path,
            category,
            tags: args
                .get_many::<String>("tags")
                .map(|s| s.map(|s| s.as_str()).collect()),
        })
    }

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        // Create the category if it does not exist
        if self.category.is_some() {
            let parent = self
                .path
                .parent()
                .expect("a invalid path provided at creation");
            if !std::fs::exists(parent)? {
                std::fs::create_dir_all(parent)?
            };
        }
        markdown::File::new(
            self.name,
            self.path
                .as_os_str()
                .to_str()
                .ok_or("a invalid path provided at creation")?,
            self.category,
            self.tags.as_deref(),
        )
        .write()
    }
}

/// Validates that a filename is represented in the format
/// <name>+([-]<name>+)?* of only ascii chars
fn validate_name(name: &str) -> Result<&str, Box<dyn Error>> {
    if let Some(p) = name.find(|c: char| !(!c.is_ascii() || c.is_alphabetic() || c == '-')) {
        return Err(format!(
            "invalid character '{}' found in filename at position {}",
            name.as_bytes()
                .get(p)
                .copied()
                .expect("p is a valid byte index into name") as char,
            p + 1
        )
        .into());
    };
    Ok(name)
}

/// Validates that a Path is in the form of /path/to/category,
/// where all characters are valid ascii alphanumeric characters or
/// underscores.
fn validate_path(path: &Path) -> Result<&str, Box<dyn Error>> {
    path.iter()
        .try_for_each(|s: &OsStr| -> Result<(), Box<dyn Error>> {
            let st = s.to_str().ok_or(format!(
                "create: found invalid UTF-8 string: {}",
                s.to_string_lossy()
            ))?;
            if let Some(p) = st.find(|c: char| !(c.is_alphanumeric() || c == '_')) {
                return Err(format!(
                    "invalid character '{}' found in {st} for {}",
                    st.as_bytes()
                        .get(p)
                        .copied()
                        .expect("p is a valid byte index into name") as char,
                    path.to_string_lossy()
                )
                .into());
            };
            Ok(())
        })?;
    Ok(path.to_str().unwrap())
}
