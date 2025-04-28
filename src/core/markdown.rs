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

    // FIXME: Write proper error handling
    pub fn write(&self) -> Result<(), ()> {
        unimplemented!("File writing pending implementation")
    }
}
