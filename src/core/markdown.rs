pub struct File<'a> {
    directory: Option<&'a str>,
    tags: Option<&'a [&'a str]>,
}

impl<'a> File<'a> {
    pub(crate) fn new(directory: Option<&'a str>, tags: Option<&'a [&'a str]>) -> Self {
        Self { directory, tags }
    }

    // FIXME: Write proper error handling
    pub fn create_file(&self) -> Result<(), ()> {
        Ok(())
    }
}
