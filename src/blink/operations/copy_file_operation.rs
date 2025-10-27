use crate::blink::operations::Operation;
use std::{fs, io, path::PathBuf};

#[derive(Debug)]
pub struct CopyFile {
    old_path: PathBuf,
    dst_path: PathBuf,
    copy_path: PathBuf,
    executed: bool,
}

impl CopyFile {
    pub fn new(old_path: PathBuf, dst_path: PathBuf) -> Self {
        Self {
            old_path,
            dst_path,
            copy_path: PathBuf::default(),
            executed: false,
        }
    }
}

impl Operation for CopyFile {
    fn execute(&mut self) -> io::Result<()> {
        if let Some(filename) = self.old_path.file_name() {
            self.copy_path = self.dst_path.join(filename);
            if self.copy_path.exists() {
                let stem = self
                    .copy_path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let ext = self
                    .copy_path
                    .extension()
                    .map(|e| format!(".{}", e.to_string_lossy()))
                    .unwrap_or_default();
                let mut counter = 1;
                loop {
                    self.copy_path = self
                        .dst_path
                        .join(format!("{}_copy{}{}", stem, counter, ext));
                    if !self.copy_path.exists() {
                        break;
                    }
                    counter += 1;
                }
            }
            if self.old_path.is_dir() {
                copy_directory_recursively(&self.old_path, &self.copy_path)?;
            } else {
                fs::copy(&self.old_path, &self.copy_path)?;
            }
        }
        self.executed = true;
        Ok(())
    }

    fn undo(&self) -> io::Result<()> {
        if self.executed && self.copy_path.exists() {
            if self.copy_path.is_dir() {
                fs::remove_dir_all(self.copy_path.clone())?;
            } else {
                fs::remove_file(self.copy_path.clone())?;
            }
        }
        Ok(())
    }
}

fn copy_directory_recursively(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_directory_recursively(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
