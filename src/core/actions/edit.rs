use clap::ArgMatches;

use crate::core::markdown;

use super::{Command, Configuration};

/// Representation of an edit command and the context
/// needed for an edit command
pub struct Edit {
    name: String,
    path: String,
    category: Option<String>,
    tags: Option<Vec<String>>,
}

impl Command for Edit {
    fn new(_args: &ArgMatches, conf: &Configuration) -> Self {
        // We always demand an argument name
        let name = _args.get_one::<String>("name").unwrap().clone();
        let category = _args.get_one::<String>("category").cloned();
        let directory = if let Some(category) = category.as_deref() {
            format!(
                "{}/{}/{}.md",
                conf.settings.path.to_owned(),
                category,
                name.replace(" ", "-").trim()
            )
        } else {
            format!(
                "{}/{}.md",
                conf.settings.path,
                name.replace(" ", "-").trim()
            )
        };

        Self {
            name,
            category,
            path: directory,
            tags: None,
        }
    }

    fn execute(&self) -> Result<(), ()> {
        let file = markdown::File::new(
            self.name.as_ref(),
            self.path.as_str(),
            self.category.as_deref(),
            self.tags.as_deref(),
        );
        // FIXME: Implement proper error handling
        file.write().map_err(|_| ())
    }
}
