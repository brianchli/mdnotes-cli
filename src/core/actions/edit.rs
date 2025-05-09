use std::error::Error;

use clap::ArgMatches;

use crate::core::markdown;

use super::{Command, Configuration};

/// Representation of an edit command and the context
/// needed for an edit command
pub struct Edit<'a> {
    name: &'a str,
    path: String,
    category: Option<&'a str>,
    tags: Option<Vec<&'a str>>,
}

impl<'a> Command<'a> for Edit<'a> {
    fn new(args: &'a ArgMatches, conf: &Configuration) -> Result<Self, Box<dyn Error>> {
        // We always demand an argument name
        let name = args.get_one::<String>("name").unwrap();
        let category = args.get_one::<String>("category").map(|s| s.as_str());
        // FIXME: Validation for categories. They should be in the format of
        // a relative directory path e.g., /some/path/
        let mut path = if let Some(category) = category {
            format!("{}/{}", conf.settings.path, category)
        } else {
            conf.settings.path.clone()
        };
        for p in name.split("/") {
            if !p.is_empty() {
                path.push('/');
                path.push_str(p);
            }
        }
        path.push_str(".md");
        Ok(Self {
            name: name.as_str(),
            category,
            path,
            tags: args
                .get_many::<String>("tags")
                .map(|s| s.map(|s| s.as_str()).collect()),
        })
    }

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        // Create the category if it does not exist
        if self.category.is_some() {
            // The path will always be valid
            let (parent, _) = self
                .path
                .rsplit_once("/")
                .expect("create: a non valid path provided at creation");
            if !std::fs::exists(parent)? {
                std::fs::create_dir_all(parent)?
            };
        }
        markdown::File::new(
            self.name,
            self.path.as_str(),
            self.category,
            self.tags.as_deref(),
        )
        .write()
    }
}
