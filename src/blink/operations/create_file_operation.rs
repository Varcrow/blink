use crate::blink::operations::Operation;
use std::{fs, io, path::PathBuf};

#[derive(Debug)]
pub struct CreateFile {
    path: PathBuf,
    created: bool,
}

impl CreateFile {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            created: false,
        }
    }
}

impl Operation for CreateFile {
    fn execute(&mut self) -> io::Result<()> {
        if let Some(_) = self.path.extension() {
            fs::File::create(&self.path)?;
        } else {
            fs::create_dir_all(&self.path)?;
        }
        self.created = true;
        Ok(())
    }

    fn undo(&self) -> io::Result<()> {
        if self.created && self.path.exists() {
            if self.path.is_dir() {
                fs::remove_dir_all(self.path.clone())?;
            } else {
                fs::remove_file(self.path.clone())?;
            }
        }
        Ok(())
    }
}
