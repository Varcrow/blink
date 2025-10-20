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

pub fn get_entries(show_hidden: bool, path: &Path) -> io::Result<Vec<FileEntry>> {
    // check if called path is readable otherwise SKIPPPPP
    // jk we return empty vec
    // basically a precaution since this should never happen
    // but shit would crash without it
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
        // if this is a directory we cant read then we shouldnt see it so SKIP
        if is_dir {
            if fs::read_dir(&path).is_err() {
                continue;
            }
        }
        // supposed to check if file is utf8 based on name but i dont think it's working :D
        // ill leave it here for now
        let name = match entry.file_name().to_str() {
            Some(n) => n.to_string(),
            None => continue,
        };
        let is_hidden = name.starts_with('.');
        if is_hidden == true && show_hidden == false {
            continue;
        }

        entries.push(FileEntry {
            path,
            name,
            is_dir,
            is_hidden,
        });
    }

    // sort so dirs are first followed by files and each section is alphabetical
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}
