use std::{
    error::Error,
    ffi::OsStr,
    ops::ControlFlow,
    path::{Path, PathBuf},
};

use clap::ArgMatches;

use crate::core::markdown;

const DEFAULT_EDITOR: &str = "vim";

use super::{Command, Configuration};
const EDITOR: Option<&str> = option_env!("EDITOR");

/// Representation of an edit command and the context
/// needed for an edit command
pub struct Edit<'a> {
    name: &'a str,
    path: PathBuf,
    category: Option<&'a str>,
    tags: Option<Vec<&'a str>>,
    editor: Option<&'a str>,
}

impl<'a> Command<'a> for Edit<'a> {
    fn new(args: &'a ArgMatches, conf: &'a Configuration) -> Result<Self, Box<dyn Error>> {
        // We always demand an argument name
        let name = args.get_one::<String>("name").unwrap();
        let category = args.get_one::<String>("category").map(|s| s.as_str());
        let mut path = if let Some(category) = category {
            PathBuf::from(format!(
                "{}/{}",
                conf.settings.path,
                validate_path(category)?
            ))
        } else {
            PathBuf::from(&conf.settings.path)
        };

        path.push(format!("{name}.md"));

        let editor = if args.get_one::<bool>("quiet").is_some() {
            None
        } else {
            Some(
                conf.settings
                    .editor
                    .as_deref()
                    .or(EDITOR)
                    .unwrap_or(DEFAULT_EDITOR),
            )
        };

        Ok(Self {
            name: validate_name(name)?,
            path,
            category,
            tags: args
                .get_many::<String>("tags")
                .map(|s| s.map(|s| s.as_str()).collect()),
            editor,
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
        .write()?;

        if let Some(editor) = self.editor {
            std::process::Command::new(editor)
                .arg(&self.path)
                .status()?;
        }
        Ok(())
    }
}

/// Validates that a filename is represented in the format
/// <name>+([-]<name>+)?* of only ascii chars
fn validate_name(name: &str) -> Result<&str, Box<dyn Error>> {
    if let Some(p) = name.find(|c: char| !(c.is_alphabetic() && c < 128 as char || c == '-')) {
        let res = name.chars().try_fold((0usize, 0), |(idx, b), elem| {
            if b >= p {
                ControlFlow::Break((idx, b))?;
            }
            ControlFlow::Continue((idx + 1, b + elem.len_utf8()))
        });

        let idx = res
            .continue_value()
            .map_or(res.break_value().unwrap().0, |(v, _)| v);

        return Err(format!(
            "invalid character '{}' found in filename at position {}",
            name.chars()
                .nth(idx)
                .expect("p is a valid byte index into name"),
            idx
        )
        .into());
    };
    Ok(name)
}

// FIXME: Review. May not provided sufficient guarantees.
// Portable characters can be found in definitions within
// https://pubs.opengroup.org/onlinepubs/9799919799/
fn validate_path(path: &str) -> Result<&str, Box<dyn Error>> {
    let path = Path::new(path);
    path.iter()
        .try_for_each(|s: &OsStr| -> Result<(), Box<dyn Error>> {
            let st = s.to_str().ok_or(format!(
                "create: found invalid UTF-8 string: {}",
                s.to_string_lossy()
            ))?;
            if let Some(p) = st.find(|c: char| !(c.is_ascii() || c <= 127 as char)) {
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
