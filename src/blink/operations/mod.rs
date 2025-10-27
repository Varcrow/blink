use std::io;
use std::path::PathBuf;

pub mod create_file_operation;
pub mod delete_file_operation;
pub mod rename_file_operation;

// Trait for undoable file operations
pub trait Operation: std::fmt::Debug {
    fn execute(&mut self) -> io::Result<()>;
    fn undo(&self) -> io::Result<()>;
    fn description(&self) -> String;
}

// Main undoable file system manager
struct UndoableFileSystem {
    history: Vec<Box<dyn Operation>>,
    max_history: usize,
    trash_manager: TrashManager,
}

impl UndoableFileSystem {
    fn new(max_history: usize) -> io::Result<Self> {
        Ok(Self {
            history: Vec::new(),
            max_history,
            trash_manager: TrashManager::new()?,
        })
    }

    fn execute(&mut self, mut op: Box<dyn Operation>) -> io::Result<()> {
        op.execute()?;
        println!("Executed: {}", op.description());

        self.history.push(op);

        // Limit history size
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        Ok(())
    }

    fn undo(&mut self) -> io::Result<()> {
        if let Some(op) = self.history.pop() {
            println!("Undoing: {}", op.description());
            op.undo()?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Nothing to undo"))
        }
    }

    fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    fn history_size(&self) -> usize {
        self.history.len()
    }

    fn empty_trash(&self) -> io::Result<()> {
        self.trash_manager.empty_trash()
    }

    // Convenience methods
    fn create_file(&mut self, path: PathBuf, content: Vec<u8>) -> io::Result<()> {
        let op = Box::new(CreateFile::new(path, content));
        self.execute(op)
    }

    fn delete_file(&mut self, path: PathBuf) -> io::Result<()> {
        let trash_manager = TrashManager::new()?;
        let op = Box::new(DeleteFile::new(path, trash_manager));
        self.execute(op)
    }

    fn modify_file(&mut self, path: PathBuf, new_content: Vec<u8>) -> io::Result<()> {
        let op = Box::new(ModifyFile::new(path, new_content));
        self.execute(op)
    }

    fn rename_file(&mut self, old_path: PathBuf, new_path: PathBuf) -> io::Result<()> {
        let op = Box::new(RenameFile::new(old_path, new_path));
        self.execute(op)
    }
}
