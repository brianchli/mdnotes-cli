use std::error::Error;

use serde::{Deserialize, Serialize};

pub const DATA_DIR: &str = concat!(std::env!("HOME"), "/.local/share/notes");
const CONFIG_DIR: &str = concat!(std::env!("HOME"), "/.config/notes");
pub const CONFIG_FILE: &str = concat!(std::env!("HOME"), "/.config/notes/notes.toml");

#[derive(Deserialize, Serialize, Default)]
#[allow(unused)]
pub struct Configuration {
    pub(crate) settings: Settings,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Settings {
    pub(crate) path: String,
    pub(crate) editor: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            path: DATA_DIR.to_owned(),
            editor: None,
        }
    }
}

/// Setup the requirements for the notes app:
/// * File directory location.
/// * Notes configuration file.
///
pub fn notes_init() -> Result<Configuration, Box<dyn Error>> {
    configuration_init()
}

/// Creates a valid configuration file representation on success
/// or otherwise an error.
fn configuration_init() -> Result<Configuration, Box<dyn Error>> {
    if !std::fs::exists(CONFIG_FILE)? {
        std::fs::create_dir_all(CONFIG_DIR)?;
        let conf = Configuration::default();
        let toml = toml::to_string(&conf)?;
        std::fs::write(CONFIG_FILE, &toml)?;
        return Ok(conf);
    };
    let conf = std::fs::read_to_string(CONFIG_FILE)?;
    Ok(toml::from_str::<Configuration>(conf.as_ref())?)
}
