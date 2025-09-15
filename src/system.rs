use std::{error::Error, path::Path};

use std::sync::LazyLock;

static HOME: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOME").expect("no home directory is set"));

pub static DATA_DIR: LazyLock<String> = LazyLock::new(|| {
    Path::new(&*HOME)
        .join(".local/share/notes")
        .to_string_lossy()
        .to_string()
});

pub static CONFIG_DIR: LazyLock<String> = LazyLock::new(|| {
    Path::new(&*HOME)
        .join(".config/notes")
        .to_string_lossy()
        .to_string()
});

pub static CONFIG_FILE: LazyLock<String> = LazyLock::new(|| {
    Path::new(&*HOME)
        .join(".config/notes/notes.toml")
        .to_string_lossy()
        .to_string()
});

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[allow(unused)]
pub struct Configuration {
    pub(crate) settings: Settings,
    pub(crate) options: Option<Options>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Settings {
    pub(crate) path: String,
    pub(crate) editor: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
pub(crate) struct Options {
    pub(crate) hide_root: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            path: (*DATA_DIR).to_owned(),
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
    if !std::fs::exists(Path::new(&*CONFIG_DIR))? {
        std::fs::create_dir_all(&*CONFIG_DIR)?;
        let conf = Configuration::default();
        let toml = toml::to_string(&conf)?;
        std::fs::write(&*CONFIG_FILE, &toml)?;
        return Ok(conf);
    };
    let conf = std::fs::read_to_string(&*CONFIG_FILE)?;
    Ok(toml::from_str::<Configuration>(conf.as_ref())?)
}
