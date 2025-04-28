use std::error::Error;

pub struct File<'a> {
    name: &'a str,
    path: &'a str,
    category: Option<&'a str>,
    tags: Option<&'a [String]>,
}

impl<'a> File<'a> {
    pub(crate) fn new(
        name: &'a str,
        directory: &'a str,
        category: Option<&'a str>,
        tags: Option<&'a [String]>,
    ) -> Self {
        Self {
            name,
            path: directory,
            category,
            tags,
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        unimplemented!("File writing pending implementation")
    }
}
