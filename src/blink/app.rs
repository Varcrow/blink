use crate::blink::{
    bookmarks::Bookmarks,
    config::config::Config,
    entries::{FileEntry, get_entries},
    operations::OperationManager,
    states::{main_state::MainState, state_trait::State},
    thread_pool::ThreadPool,
};
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    widgets::{Clear, ListState},
};
use std::sync::Arc;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Clone)]
pub enum Preview {
    File { contents: String },
    Directory { entries: Vec<FileEntry> },
    Image { path: PathBuf },
    Binary { info: String },
}

impl Default for Preview {
    fn default() -> Self {
        Self::Binary {
            info: String::new(),
        }
    }
}

pub struct App {
    pub running_state: RunningState,
    pub state: Box<dyn State>,
    pub operation_manager: OperationManager,
    thread_pool: ThreadPool,
    pub list_state: ListState,
    pub cwd: PathBuf,
    pub yanked_entry_paths: Option<Vec<PathBuf>>,
    pub is_cut: bool,
    pub parent_dir_entries: Vec<FileEntry>,
    pub cwd_entries: Vec<FileEntry>,
    pub visual_mode: bool,
    pub visual_anchor: Option<usize>,
    pub visual_selection: Vec<usize>,
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
            list_state: ListState::default().with_selected(Some(0)),
            cwd: path.clone(),
            yanked_entry_paths: None,
            is_cut: false,
            parent_dir_entries: Vec::new(),
            cwd_entries: Vec::new(),
            visual_mode: false,
            visual_anchor: None,
            visual_selection: Vec::new(),
            operation_manager: OperationManager::new(50)?,
            thread_pool: ThreadPool::new(1, 1024),
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
            if event::poll(Duration::from_millis(16))? {
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

    fn preload_previews(&mut self) {
        let entries = self.cwd_entries.clone();
        for (_, entry) in entries.iter().enumerate() {
            let path = entry.path.clone();
            let is_dir = entry.is_dir;
            let preview = Arc::clone(&entry.preview);

            let _ = self.thread_pool.try_execute(move || {
                let new_preview = if is_dir {
                    load_directory_preview(true, &path)
                } else {
                    load_file_preview(&path)
                };
                if let Ok(mut p) = preview.lock() {
                    *p = new_preview;
                }
            });
        }
    }

    fn update_all_entries(&mut self) {
        self.update_cwd_entries();
        self.update_parent_dir_entries();
        self.preload_previews();
    }

    pub fn toggle_visual_mode(&mut self) {
        if self.visual_mode {
            self.visual_mode = false;
            self.visual_anchor = None;
            self.visual_selection.clear();
        } else {
            self.visual_mode = true;
            self.visual_anchor = self.list_state.selected();
            if let Some(anchor) = self.visual_anchor {
                self.visual_selection = vec![anchor];
            }
        }
    }

    pub fn update_visual_selection(&mut self) {
        if let (Some(anchor), Some(current)) = (self.visual_anchor, self.list_state.selected()) {
            let start = anchor.min(current);
            let end = anchor.max(current);
            self.visual_selection = (start..=end).collect();
        }
    }

    pub fn yank_current_selection(&mut self, cut: bool) {
        if self.visual_mode {
            let paths: Vec<PathBuf> = self
                .visual_selection
                .iter()
                .filter_map(|&idx| self.cwd_entries.get(idx))
                .map(|entry| entry.path.clone())
                .collect();
            self.yanked_entry_paths = Some(paths);
            self.is_cut = cut;
        } else {
            if let Some(i) = self.list_state.selected() {
                if let Some(entry) = self.cwd_entries.get(i) {
                    self.yanked_entry_paths = Some(vec![entry.path.clone()]);
                    self.is_cut = cut;
                }
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cwd_entries.len() == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.cwd_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            _ => 0,
        };
        self.list_state.select(Some(i));

        if self.visual_mode {
            self.update_visual_selection();
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cwd_entries.len() == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.cwd_entries.len() - 1
                } else {
                    i - 1
                }
            }
            _ => 0,
        };
        self.list_state.select(Some(i));

        if self.visual_mode {
            self.update_visual_selection();
        }
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

    pub fn create_bookmark(&mut self, name: String) -> color_eyre::Result<()> {
        self.bookmarks.add(name, self.cwd.clone());
        self.bookmarks.save()
    }

    pub fn delete_bookmark(&mut self, index: usize) -> color_eyre::Result<()> {
        if let Some((tag, _)) = self.bookmarks.list().get(index) {
            self.bookmarks.remove(tag.to_string());
            let _ = self.bookmarks.save();
        }
        Ok(())
    }

    pub fn jump_to_bookmark(&mut self, index: usize) {
        if let Some((_, bookmark)) = self.bookmarks.list().get(index) {
            if bookmark.path.exists() {
                self.update_cwd(bookmark.path.clone());
            }
        }
    }

    // checks if given path is a directory which would be list_state.selected()
    // if its a directory, set the terminal app current dir to it otherwise use cwd
    // Requires EDITOR or VISUAL variable to exist to open with editor
    pub fn open_in_editor(&mut self) -> color_eyre::Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if let Ok(editor) = std::env::var("EDITOR").or_else(|_| std::env::var("VISUAL")) {
                    // check if editor is a terminal editor
                    let editor_lower = editor.to_lowercase();
                    let terminal_editors = ["vi", "vim", "nvim", "nano", "emacs", "micro", "helix"];
                    if terminal_editors.iter().any(|&e| editor_lower.contains(e)) {
                        self.open_with_terminal(&editor, &entry.path.clone())?;
                    } else {
                        open::with(&entry.path.clone(), editor)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn open_in_default_app(&mut self) -> color_eyre::Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if !entry.is_dir {
                    open::that_in_background(&entry.path);
                }
            }
        }

        Ok(())
    }

    // Drops into the terminal editor from blink and returns to blink once the editor closes
    // checks if given path is a directory which would be list_state.selected()
    // if its a directory, set the terminal app current dir to it, otherwise use cwd
    fn open_with_terminal(
        &mut self,
        editor: &str,
        path: &std::path::Path,
    ) -> color_eyre::Result<()> {
        ratatui::restore();
        _ = Command::new(editor)
            .arg(path)
            .current_dir(if path.is_dir() { path } else { &self.cwd })
            .status();

        std::thread::sleep(std::time::Duration::from_millis(50));
        let mut terminal = ratatui::init();
        terminal.clear()?;
        self.update_all_entries();
        terminal.draw(|frame| {
            frame.render_widget(Clear, frame.area());
            self.state.render(self, frame);
        })?;

        Ok(())
    }
}

// Operation functions
impl App {
    pub fn undo_last_operation(&mut self) {
        _ = self.operation_manager.undo();
        self.update_all_entries();
    }

    pub fn create_file(&mut self, name: &str) {
        _ = self.operation_manager.create_file(self.cwd.join(name));
        self.update_all_entries();
    }

    pub fn rename_current_selected_path(&mut self, new_name: &str) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                _ = self
                    .operation_manager
                    .rename_file(entry.path.clone(), self.cwd.join(new_name));
                self.update_all_entries();
            }
        }
    }

    pub fn delete_current_selection(&mut self) {
        if self.visual_mode && !self.visual_selection.is_empty() {
            for &idx in &self.visual_selection {
                if let Some(entry) = self.cwd_entries.get(idx) {
                    _ = self.operation_manager.delete_file(entry.path.clone());
                }
            }
            self.move_cursor_up();
            self.update_all_entries();
        } else {
            if let Some(i) = self.list_state.selected() {
                if let Some(entry) = self.cwd_entries.get(i) {
                    _ = self.operation_manager.delete_file(entry.path.clone());

                    if i != 0 {
                        self.move_cursor_up();
                    }
                    self.update_all_entries();
                }
            }
        }
    }

    pub fn paste_yanked_path(&mut self) {
        if let Some(sources) = &self.yanked_entry_paths.clone() {
            if self.is_cut {
                for source in sources {
                    if let Some(filename) = source.file_name() {
                        _ = self
                            .operation_manager
                            .rename_file(source.clone(), self.cwd.join(filename).clone())
                    }
                }
            } else {
                for source in sources {
                    _ = self
                        .operation_manager
                        .copy_file(source.clone(), self.cwd.clone())
                }
            }
            self.yanked_entry_paths = None;
            self.is_cut = false;
            self.update_all_entries();
        }
    }
}

fn load_directory_preview(show_hidden: bool, path: &Path) -> Preview {
    Preview::Directory {
        entries: get_entries(show_hidden, path).unwrap_or_default(),
    }
}

fn load_file_preview(path: &Path) -> Preview {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        if matches!(
            ext_lower.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp"
        ) {
            return Preview::Image {
                path: path.to_path_buf(),
            };
        }
    }

    match fs::read(path) {
        Ok(bytes) => {
            let check_len = bytes.len().min(8192);
            if bytes[..check_len].contains(&0) {
                Preview::Binary {
                    info: format!("Binary file ({} bytes)", bytes.len()),
                }
            } else {
                let preview_bytes = if bytes.len() > 1_000_000 {
                    &bytes[..1_000_000]
                } else {
                    &bytes
                };
                Preview::File {
                    contents: String::from_utf8_lossy(preview_bytes).to_string(),
                }
            }
        }
        Err(_) => Preview::File {
            contents: "[Cannot read file]".to_string(),
        },
    }
}
