use color_eyre::eyre::Ok;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bookmarks {
    bookmarks: HashMap<String, BookmarkEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub path: PathBuf,
}

impl Bookmarks {
    pub fn load() -> color_eyre::Result<Self> {
        let path = Self::bookmarks_path()?;

        if !path.exists() {
            let default = Self::default();
            default.save()?;
            return Ok(default);
        }

        let contents = fs::read_to_string(&path)?;
        let bookmarks: Bookmarks = toml::from_str(&contents)?;
        Ok(bookmarks)
    }

    pub fn save(&self) -> color_eyre::Result<()> {
        let path = Self::bookmarks_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string_pretty(self)?;
        fs::write(&path, toml_string)?;
        Ok(())
    }

    fn bookmarks_path() -> color_eyre::Result<PathBuf> {
        use directories::ProjectDirs;

        let proj_dirs = ProjectDirs::from("com", "Varcrow", "blink")
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not determine config directory"))?;

        Ok(proj_dirs.config_dir().join("bookmarks.toml"))
    }

    pub fn add(&mut self, tag: String, path: PathBuf) {
        self.bookmarks.insert(tag, BookmarkEntry { path });
    }

    pub fn remove(&mut self, tag: String) -> Option<BookmarkEntry> {
        self.bookmarks.remove(&tag)
    }

    pub fn list(&self) -> Vec<(&String, &BookmarkEntry)> {
        let mut list: Vec<_> = self.bookmarks.iter().collect();
        list.sort_by(|a, b| a.0.cmp(b.0));
        list
    }
}
