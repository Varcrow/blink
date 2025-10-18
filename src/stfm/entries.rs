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

// This func filters out the shit it can't read aka doesnt have permission for since program goes
// monke mode if we attempt to peak at shit we arent allowed to:D
pub fn get_entries(path: &Path) -> io::Result<Vec<FileEntry>> {
    // can we even read the dir
    let _ = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => {
            return Ok(Vec::new());
        }
    };

    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
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
        let name = entry.file_name().to_string_lossy().to_string();
        let is_hidden = name.starts_with('.');

        entries.push(FileEntry {
            path,
            name,
            is_dir,
            is_hidden,
        });
    }

    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}
