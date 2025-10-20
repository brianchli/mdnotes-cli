use std::{
    error::Error,
    ffi::OsStr,
    ops::ControlFlow,
    path::{Path, PathBuf},
};

use crate::core::markdown;

use super::{Command, Commands, Configuration};
const DEFAULT_EDITOR: &str = "vim";

/// Representation of an edit command and the context
/// needed for an edit command
pub struct CreateCommand<'a> {
    name: String,
    path: PathBuf,
    category: Option<String>,
    tags: Option<Vec<String>>,
    editor: Option<&'a str>,
}

impl<'a> Command<'a> for CreateCommand<'a> {
    fn new(args: Commands, conf: &'a Configuration) -> Result<Self, Box<dyn Error>> {
        // We always demand an argument name
        let Commands::Create {
            quiet,
            category,
            name,
            tags,
        } = args
        else {
            unreachable!("Non-create command provided to create handler.")
        };

        let mut path = if let Some(category) = &category {
            PathBuf::from(format!(
                "{}/{}",
                conf.settings.path,
                validate_path(category.as_str())?
            ))
        } else {
            PathBuf::from(&conf.settings.path)
        };

        path.push(format!("{name}.md"));

        let editor = if !quiet {
            Some(
                conf.settings
                    .editor
                    .as_deref()
                    .or_else(|| {
                        let mut vars = std::env::vars();
                        let allowed_editors: fn((String, String)) -> Option<&'a str> =
                            |(_, v)| match v.as_str() {
                                "nvim" => Some("nvim"),
                                "glow" => Some("glow"),
                                _ => None,
                            };
                        vars.find(|(key, _)| key == "NOTES_EDITOR")
                            .and_then(allowed_editors)
                            .and_then(|s| {
                                if s == "glow" && !vars.any(|(k, _)| k == "EDITOR") {
                                    None
                                } else {
                                    Some(s)
                                }
                            })
                    })
                    .unwrap_or(DEFAULT_EDITOR),
            )
        } else {
            None
        };

        Ok(Self {
            name: validate_name(name)?,
            path,
            category,
            tags,
            editor,
        })
    }

    fn execute(self) -> Result<(), Box<dyn Error>> {
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
            self.name.as_str(),
            self.path
                .as_os_str()
                .to_str()
                .ok_or("a invalid path provided at creation")?,
            self.category.as_deref(),
            self.tags,
        )
        .write()?;

        if let Some(editor) = self.editor {
            let mut command = std::process::Command::new(editor);
            if editor == "glow" {
                command.arg(r#"--tui"#);
            }
            command.arg(self.path.as_os_str()).status()?;
        }

        Ok(())
    }
}

/// Validates that a filename is represented in the format
/// <name>+([-]<name>+)?* of only ascii chars
fn validate_name(name: String) -> Result<String, Box<dyn Error>> {
    if let Some(p) = name.find(|c: char| !(c.is_ascii() && c < 128 as char || c == '-')) {
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
