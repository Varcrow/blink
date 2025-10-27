use crate::blink::operations::Operation;
use std::{fs, io, path::PathBuf};

#[derive(Debug)]
pub struct RenameFile {
    old_path: PathBuf,
    new_path: PathBuf,
    executed: bool,
}

impl RenameFile {
    pub fn new(old_path: PathBuf, new_path: PathBuf) -> Self {
        Self {
            old_path,
            new_path,
            executed: false,
        }
    }
}

impl Operation for RenameFile {
    fn execute(&mut self) -> io::Result<()> {
        fs::rename(&self.old_path, &self.new_path)?;
        self.executed = true;
        Ok(())
    }

    fn undo(&self) -> io::Result<()> {
        if self.executed {
            fs::rename(&self.new_path, &self.old_path)?;
        }
        Ok(())
    }
}
