use crate::blink::operations::copy_file_operation::CopyFile;
use crate::blink::operations::create_file_operation::CreateFile;
use crate::blink::operations::delete_file_operation::DeleteFile;
use crate::blink::operations::rename_file_operation::RenameFile;
use crate::blink::trash_manager::TrashManager;
use std::io;
use std::path::PathBuf;

pub mod copy_file_operation;
pub mod create_file_operation;
pub mod delete_file_operation;
pub mod rename_file_operation;

pub trait Operation: std::fmt::Debug {
    fn execute(&mut self) -> io::Result<()>;
    fn undo(&self) -> io::Result<()>;
}

pub struct OperationManager {
    history: Vec<Box<dyn Operation>>,
    max_history: usize,
    trash_manager: TrashManager,
}

impl OperationManager {
    pub fn new(max_history: usize) -> io::Result<Self> {
        Ok(Self {
            history: Vec::new(),
            max_history,
            trash_manager: TrashManager::new()?,
        })
    }

    fn execute(&mut self, mut op: Box<dyn Operation>) -> io::Result<()> {
        op.execute()?;

        self.history.push(op);

        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        Ok(())
    }

    pub fn undo(&mut self) -> io::Result<()> {
        if let Some(op) = self.history.pop() {
            op.undo()?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Nothing to undo"))
        }
    }

    /*
    fn empty_trash(&self) -> io::Result<()> {
        self.trash_manager.empty_trash()
    }
    */

    pub fn create_file(&mut self, path: PathBuf) -> io::Result<()> {
        let op = Box::new(CreateFile::new(path));
        self.execute(op)
    }

    pub fn delete_file(&mut self, path: PathBuf) -> io::Result<()> {
        let op = Box::new(DeleteFile::new(path, self.trash_manager.clone()));
        self.execute(op)
    }

    pub fn rename_file(&mut self, old_path: PathBuf, new_path: PathBuf) -> io::Result<()> {
        let op = Box::new(RenameFile::new(old_path, new_path));
        self.execute(op)
    }

    pub fn copy_file(&mut self, old_path: PathBuf, dst_path: PathBuf) -> io::Result<()> {
        let op = Box::new(CopyFile::new(old_path, dst_path));
        self.execute(op)
    }
}
