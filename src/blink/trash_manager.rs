use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct TrashManager {
    trash_dir: PathBuf,
}

impl TrashManager {
    pub fn new() -> io::Result<Self> {
        let trash_dir = Self::get_trash_directory()?;
        if !trash_dir.exists() {
            fs::create_dir_all(&trash_dir)?;
        }
        Ok(Self { trash_dir })
    }

    #[cfg(target_os = "windows")]
    fn get_trash_directory() -> io::Result<PathBuf> {
        // Windows: Use AppData\Local\Temp\app_trash
        let local_app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
        Ok(PathBuf::from(local_app_data).join("app_trash"))
    }

    #[cfg(target_os = "macos")]
    fn get_trash_directory() -> io::Result<PathBuf> {
        // macOS: Use ~/.Trash/app_trash
        let home = std::env::var("HOME")
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME not set"))?;
        Ok(PathBuf::from(home).join(".Trash").join("app_trash"))
    }

    #[cfg(target_os = "linux")]
    fn get_trash_directory() -> io::Result<PathBuf> {
        // Linux: Use XDG trash or fallback to ~/.local/share/Trash
        if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
            Ok(PathBuf::from(xdg_data).join("Trash").join("files"))
        } else if let Ok(home) = std::env::var("HOME") {
            Ok(PathBuf::from(home).join(".local/share/Trash/files"))
        } else {
            Ok(std::env::temp_dir().join("app_trash"))
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    fn get_trash_directory() -> io::Result<PathBuf> {
        // Fallback for other platforms
        Ok(std::env::temp_dir().join("app_trash"))
    }

    fn generate_trash_path(&self, original_path: &Path) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let filename = original_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        self.trash_dir.join(format!("{}_{}", timestamp, filename))
    }

    pub fn move_to_trash(&self, path: &Path) -> io::Result<PathBuf> {
        let trash_path = self.generate_trash_path(path);
        fs::rename(path, &trash_path)?;
        Ok(trash_path)
    }

    pub fn restore_from_trash(&self, trash_path: &Path, original_path: &Path) -> io::Result<()> {
        fs::rename(trash_path, original_path)
    }

    pub fn empty_trash(&self) -> io::Result<()> {
        if self.trash_dir.exists() {
            fs::remove_dir_all(&self.trash_dir)?;
            fs::create_dir_all(&self.trash_dir)?;
        }
        Ok(())
    }
}
