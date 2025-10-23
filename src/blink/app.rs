use crate::blink::{
    bookmarks::Bookmarks,
    config::Config,
    entries::{FileEntry, get_entries},
    states::{MainState, State},
};
use crossterm::terminal;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    widgets::{Clear, ListState},
};
use std::{fs, path::PathBuf, process::Command, time::Duration};
use std::{io, path::Path};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum Preview {
    File { contents: String },
    Directory { entries: Vec<FileEntry> },
}

// wtf is this
impl Default for Preview {
    fn default() -> Self {
        Self::File {
            contents: String::new(),
        }
    }
}

pub struct App {
    pub running_state: RunningState,
    pub state: Box<dyn State>,
    pub list_state: ListState,
    pub cwd: PathBuf,
    pub yanked_entry_path: Option<PathBuf>,
    pub is_cut: bool,
    pub parent_dir_entries: Vec<FileEntry>,
    pub cwd_entries: Vec<FileEntry>,
    pub preview_contents: Preview,
    pub bookmarks: Bookmarks,
    pub config: Config,
}

impl App {
    pub fn new(path: PathBuf) -> color_eyre::Result<App> {
        let config = Config::load()?;
        let bookmarks = Bookmarks::load()?;
        let mut app = App {
            running_state: RunningState::Running,
            state: Box::new(MainState),
            list_state: ListState::default(),
            cwd: path.clone(),
            yanked_entry_path: None,
            is_cut: false,
            parent_dir_entries: Vec::new(),
            cwd_entries: Vec::new(),
            preview_contents: Preview::File {
                contents: String::new(),
            },
            bookmarks,
            config,
        };
        app.update_all_entries();
        Ok(app)
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        let mut terminal = ratatui::init();
        while self.running_state != RunningState::Done {
            terminal.draw(|frame| {
                frame.render_widget(Clear, frame.area());
                self.state.render(self, frame)
            })?;
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        let old_state = std::mem::replace(&mut self.state, Box::new(MainState));
                        self.state = old_state.handle_input(key.code, self);
                    }
                }
            }
        }
        ratatui::restore();
        Ok(())
    }

    fn update_cwd(&mut self, path: PathBuf) {
        self.cwd = path;
        self.list_state.select(Some(0));
        self.update_all_entries();
    }

    fn update_cwd_entries(&mut self) {
        self.cwd_entries =
            get_entries(self.config.ui.show_hidden, self.cwd.as_path()).unwrap_or_default();
    }

    fn update_parent_dir_entries(&mut self) {
        if let Some(parent) = self.cwd.parent() {
            self.parent_dir_entries =
                get_entries(self.config.ui.show_hidden, parent).unwrap_or_default();
        } else {
            self.parent_dir_entries.clear();
        }
    }

    fn update_preview_contents(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(selected_idx) {
                if entry.is_dir {
                    let entries =
                        get_entries(self.config.ui.show_hidden, &entry.path).unwrap_or_default();
                    self.preview_contents = Preview::Directory { entries }
                } else {
                    let contents = fs::read_to_string(&entry.path)
                        .unwrap_or_else(|_| "[Binary file or non-UTF-8 content]".to_string());
                    self.preview_contents = Preview::File { contents }
                }
            }
        }
    }

    fn update_all_entries(&mut self) {
        self.update_cwd_entries();
        self.update_parent_dir_entries();
        self.update_preview_contents();
    }

    pub fn new_path(&mut self, name: &str) {
        let new_path = self.cwd.join(name);
        if name.contains('.') {
            fs::File::create(&new_path);
        } else {
            fs::create_dir_all(&new_path);
        }
        self.update_all_entries();
    }

    pub fn rename_current_selected_path(&mut self, new_name: &str) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                let new_path = self.cwd.join(new_name);
                fs::rename(&entry.path, &new_path);
                self.update_all_entries();
            }
        }
    }

    pub fn delete_current_selection(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if entry.is_dir {
                    fs::remove_dir_all(&entry.path);
                } else {
                    fs::remove_file(&entry.path);
                }
                self.update_all_entries();
            }
        }
    }

    pub fn yank_current_selection(&mut self, cut: bool) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                self.yanked_entry_path = Some(entry.path.clone());
                self.is_cut = cut;
            }
        }
    }

    pub fn paste_yanked_path(&mut self) {
        if let Some(source) = &self.yanked_entry_path {
            if let Some(filename) = source.file_name() {
                let mut destination = self.cwd.join(filename);

                if destination.exists() {
                    let stem = destination
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();
                    let ext = destination
                        .extension()
                        .map(|e| format!(".{}", e.to_string_lossy()))
                        .unwrap_or_default();

                    let mut counter = 1;
                    loop {
                        destination = self.cwd.join(format!("{}_copy{}{}", stem, counter, ext));
                        if !destination.exists() {
                            break;
                        }
                        counter += 1;
                    }
                }

                let result = if source.is_file() {
                    fs::copy(source, &destination).map(|_| ())
                } else if source.is_dir() {
                    self.copy_directory_recursively(source, &destination)
                } else {
                    return;
                };

                if result.is_ok() {
                    if self.is_cut {
                        if source.is_file() {
                            let _ = fs::remove_file(source);
                        } else if source.is_dir() {
                            let _ = fs::remove_dir_all(source);
                        }
                    }

                    self.yanked_entry_path = None;
                    self.is_cut = false;
                    self.update_all_entries();
                }
            }
        }
    }

    pub fn move_forward_in_cwd_list(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.cwd_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_preview_contents();
    }

    pub fn move_back_in_cwd_list(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.cwd_entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_preview_contents();
    }

    pub fn enter_current_path_selection(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if entry.is_dir {
                    self.list_state.select(Some(0));
                    self.update_cwd(entry.path.clone());
                }
            }
        }
    }

    pub fn go_up_one_directory_level(&mut self) {
        if let Some(parent) = self.cwd.parent() {
            self.update_cwd(parent.to_path_buf());
        }
    }

    pub fn create_bookmark(&mut self, name: String) {
        self.bookmarks.add(name, self.cwd.clone());
        self.bookmarks.save();
    }

    pub fn jump_to_bookmark(&mut self, index: usize) {
        let bookmarks: Vec<_> = self.bookmarks.list();
        if let Some((_, bookmark)) = bookmarks.get(index) {
            if bookmark.path.exists() {
                self.update_cwd(bookmark.path.clone());
            }
        }
    }

    pub fn copy_directory_recursively(&self, src: &Path, dst: &Path) -> io::Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                self.copy_directory_recursively(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    pub fn open_current_selection(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if !entry.is_dir {
                    self.open_file(&entry.path);
                }
            }
        }
    }

    fn open_file(&self, path: &std::path::Path) {
        ratatui::restore();

        // open the file based on OS
        #[cfg(target_os = "macos")]
        let status = Command::new("open").arg(path).status();

        #[cfg(target_os = "linux")]
        let status = Command::new("xdg-open").arg(path).status();

        #[cfg(target_os = "windows")]
        let status = Command::new("cmd")
            .args(["/C", "start", "", &path.to_string_lossy()])
            .status();

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        let status = Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Unsupported OS",
        ));

        // clear the terminal
        std::thread::sleep(std::time::Duration::from_millis(50));
        let mut terminal = ratatui::init();
        terminal.clear();
        terminal.draw(|frame| {
            frame.render_widget(Clear, frame.area());
            self.state.render(self, frame);
        });
    }
}
