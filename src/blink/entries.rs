#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[cfg(windows)]
fn is_system_or_hidden(metadata: &fs::Metadata) -> bool {
    const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;
    let attrs = metadata.file_attributes();
    attrs & FILE_ATTRIBUTE_SYSTEM != 0
}

#[cfg(not(windows))]
fn is_system_or_hidden(_metadata: &fs::Metadata) -> bool {
    false
}

pub fn get_entries(show_hidden: bool, path: &Path) -> io::Result<Vec<FileEntry>> {
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

        // Filter out Windows system files early
        if is_system_or_hidden(&metadata) {
            continue;
        }

        let name = match entry.file_name().to_str() {
            Some(n) => n.to_string(),
            _ => continue,
        };

        let is_hidden = name.starts_with('.');
        if !show_hidden && is_hidden {
            continue;
        }

        let path = entry.path();
        let is_dir = metadata.is_dir();
        let size = metadata.len();

        entries.push(FileEntry {
            path,
            name,
            is_dir,
            size,
        });
    }

    // Sort so dirs are first followed by files and each section is alphabetical
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}
