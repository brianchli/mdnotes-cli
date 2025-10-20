use std::path::PathBuf;

pub fn disallow_operation_on_active_notebook(
    p: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut notes_base = PathBuf::from(
        p.parent()
            .ok_or("Failed to fetch parent in notebook command")?
            .parent()
            .ok_or("Failed to fetch parent in notebook notebook")?,
    );
    notes_base.push(".notes");
    let buf = std::fs::read_to_string(notes_base)?;
    let active_notebook = buf
        .split_once("notebook: ")
        .ok_or("unable to get active notebook in notebook command.")?
        .1;
    let notebook = p
        .file_name()
        .ok_or("unable to get filename in notebook command")?;
    if active_notebook.trim_end() == notebook {
        Err(format!(
            "cannot apply operation as '{}' is the current active notebook",
            notebook.to_string_lossy()
        )
        .into())
    } else {
        Ok(p)
    }
}

pub fn disallow_reserved_names(p: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if p.file_name()
        .ok_or("unable to get file_name for notebook command")?
        == "main"
    {
        Err("notebook operation cannot executed on 'main'".into())
    } else {
        Ok(p)
    }
}

pub fn disallow_files_with_extensions(p: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if p.extension().is_some() {
        Err(format!(
            "'{}' cannot be used for the notebook operation",
            p.file_name()
                .ok_or("unable to get file name for notebook command")?
                .to_string_lossy()
        )
        .into())
    } else {
        Ok(p)
    }
}
