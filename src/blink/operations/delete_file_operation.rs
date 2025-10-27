use crate::blink::{operations::Operation, trash_manager::TrashManager};
use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct DeleteFile {
    path: PathBuf,
    trash_path: Option<PathBuf>,
    trash_manager: TrashManager,
}

impl DeleteFile {
    pub fn new(path: PathBuf, trash_manager: TrashManager) -> Self {
        Self {
            path,
            trash_path: None,
            trash_manager,
        }
    }
}

impl Operation for DeleteFile {
    fn execute(&mut self) -> io::Result<()> {
        if self.path.exists() {
            let trash_path = self.trash_manager.move_to_trash(&self.path)?;
            self.trash_path = Some(trash_path);
        }
        Ok(())
    }

    fn undo(&self) -> io::Result<()> {
        if let Some(trash_path) = &self.trash_path {
            if trash_path.exists() {
                self.trash_manager
                    .restore_from_trash(trash_path, &self.path)?;
            }
        }
        Ok(())
    }
}
