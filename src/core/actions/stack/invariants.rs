use std::path::PathBuf;

pub fn disallow_operation_on_active_note_stack(
    p: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut notes_base = PathBuf::from(
        p.parent()
            .ok_or("Failed to fetch parent in stack command")?
            .parent()
            .ok_or("Failed to fetch parent in stack command")?,
    );
    notes_base.push(".notes");
    let buf = std::fs::read_to_string(notes_base)?;
    let active_stack = buf
        .split_once("stack: ")
        .ok_or("unable to get active note stack in stack command.")?
        .1;
    let stack = p
        .file_name()
        .ok_or("unable to get filename in stack command")?;
    if active_stack.trim_end() == stack {
        Err(format!(
            "cannot apply operation as '{}' is the current active note stack",
            stack.to_string_lossy()
        )
        .into())
    } else {
        Ok(p)
    }
}

pub fn disallow_reserved_names(p: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if p.file_name()
        .ok_or("unable to get file_name for stack command")?
        == "main"
    {
        Err("stack operation cannot executed on 'main'".into())
    } else {
        Ok(p)
    }
}

pub fn disallow_files_with_extensions(p: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if p.extension().is_some() {
        Err(format!(
            "'{}' cannot be used for the stack operation",
            p.file_name()
                .ok_or("unable to get file name for stack command")?
                .to_string_lossy()
        )
        .into())
    } else {
        Ok(p)
    }
}
