use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_hidden: bool,
}

// This func filters out the shit it can't read aka doesnt have permission for
pub fn get_entries(path: &Path) -> io::Result<Vec<FileEntry>> {
    // if we cant even read the directory then return nothing
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return Ok(Vec::new()),
    };

    let mut entries = Vec::new();

    for entry in read_dir {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let path = entry.path();
        let is_dir = metadata.is_dir();
        // if this entries a directory then can we read it, if not it goes gulag
        if is_dir {
            if fs::read_dir(&path).is_err() {
                continue;
            }
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let is_hidden = name.starts_with('.');

        entries.push(FileEntry {
            path,
            name,
            is_dir,
            is_hidden,
        });
    }

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}
