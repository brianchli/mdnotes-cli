use serde::{Deserialize, Serialize};

pub const DATA_DIR: &str = concat!(std::env!("HOME"), "/.local/share/notes");
const CONFIG_DIR: &str = concat!(std::env!("HOME"), "/.config/notes");
const CONFIG_FILE: &str = concat!(std::env!("HOME"), "/.config/notes/notes.toml");

#[derive(Deserialize, Serialize, Default)]
#[allow(unused)]
pub struct Configuration {
    pub(crate) settings: Settings,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Settings {
    pub(crate) path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            path: DATA_DIR.to_owned(),
        }
    }
}

// FIXME: Proper error messages and error handling
//
/// Setup the requirements for the notes app:
/// * File directory location.
/// * Notes configuration file.
///
pub fn notes_init() -> Result<Configuration, ()> {
    configuration_init()
}

// FIXME: Proper error messages and error handling
/// Creates a valid configuration file representation on success
/// or otherwise an error.
fn configuration_init() -> Result<Configuration, ()> {
    if !std::fs::exists(CONFIG_FILE).map_err(|_| ())? {
        std::fs::create_dir_all(CONFIG_DIR).map_err(|_| ())?;
        let conf = Configuration::default();
        let toml = toml::to_string(&conf).map_err(|_| ())?;
        std::fs::write(CONFIG_FILE, &toml).map_err(|_| ())?;
        return Ok(conf);
    };
    let conf = std::fs::read_to_string(CONFIG_FILE).map_err(|_| ())?;
    toml::from_str(conf.as_ref()).map_err(|_| ())
}
