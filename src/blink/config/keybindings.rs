use ratatui::crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Keybindings {
    // Navigation
    #[serde(default = "default_key_down")]
    pub move_down: Vec<String>,
    #[serde(default = "default_key_up")]
    pub move_up: Vec<String>,
    #[serde(default = "default_key_left")]
    pub go_back: Vec<String>,
    #[serde(default = "default_key_right")]
    pub go_forward: Vec<String>,

    // File operations
    #[serde(default = "default_key_yank")]
    pub yank: Vec<String>,
    #[serde(default = "default_key_cut")]
    pub cut: Vec<String>,
    #[serde(default = "default_key_paste")]
    pub paste: Vec<String>,
    #[serde(default = "default_key_delete")]
    pub delete: Vec<String>,
    #[serde(default = "default_key_rename")]
    pub rename: Vec<String>,
    #[serde(default = "default_key_new")]
    pub new_entry: Vec<String>,

    // Open
    #[serde(default = "default_key_open_editor")]
    pub open_editor: Vec<String>,
    #[serde(default = "default_key_open_default")]
    pub open_default: Vec<String>,

    // Selection
    #[serde(default = "default_key_visual")]
    pub visual_mode: Vec<String>,

    // Bookmarks
    #[serde(default = "default_key_bookmark_list")]
    pub bookmark_list: Vec<String>,
    #[serde(default = "default_key_bookmark_new")]
    pub bookmark_new: Vec<String>,

    // Misc
    #[serde(default = "default_key_undo")]
    pub undo: Vec<String>,
    #[serde(default = "default_key_quit")]
    pub quit: Vec<String>,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            move_down: default_key_down(),
            move_up: default_key_up(),
            go_back: default_key_left(),
            go_forward: default_key_right(),
            yank: default_key_yank(),
            cut: default_key_cut(),
            paste: default_key_paste(),
            delete: default_key_delete(),
            rename: default_key_rename(),
            new_entry: default_key_new(),
            open_editor: default_key_open_editor(),
            open_default: default_key_open_default(),
            visual_mode: default_key_visual(),
            bookmark_list: default_key_bookmark_list(),
            bookmark_new: default_key_bookmark_new(),
            undo: default_key_undo(),
            quit: default_key_quit(),
        }
    }
}

impl Keybindings {
    pub fn matches(&self, key: KeyCode, bindings: &[String]) -> bool {
        let key_str = keycode_to_string(key);
        bindings.iter().any(|binding| binding == &key_str)
    }
}

fn keycode_to_string(key: KeyCode) -> String {
    match key {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Up => "up".to_string(),
        KeyCode::Down => "down".to_string(),
        KeyCode::Left => "left".to_string(),
        KeyCode::Right => "right".to_string(),
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Esc => "esc".to_string(),
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::Delete => "delete".to_string(),
        KeyCode::Home => "home".to_string(),
        KeyCode::End => "end".to_string(),
        KeyCode::PageUp => "pageup".to_string(),
        KeyCode::PageDown => "pagedown".to_string(),
        _ => "unknown".to_string(),
    }
}

// Default keybinding functions
fn default_key_down() -> Vec<String> {
    vec!["j".to_string(), "down".to_string()]
}

fn default_key_up() -> Vec<String> {
    vec!["k".to_string(), "up".to_string()]
}

fn default_key_left() -> Vec<String> {
    vec!["h".to_string(), "left".to_string()]
}

fn default_key_right() -> Vec<String> {
    vec!["l".to_string(), "right".to_string()]
}

fn default_key_yank() -> Vec<String> {
    vec!["y".to_string()]
}

fn default_key_cut() -> Vec<String> {
    vec!["x".to_string()]
}

fn default_key_paste() -> Vec<String> {
    vec!["p".to_string()]
}

fn default_key_delete() -> Vec<String> {
    vec!["d".to_string()]
}

fn default_key_rename() -> Vec<String> {
    vec!["r".to_string()]
}

fn default_key_new() -> Vec<String> {
    vec!["m".to_string()]
}

fn default_key_open_editor() -> Vec<String> {
    vec!["e".to_string()]
}

fn default_key_open_default() -> Vec<String> {
    vec!["o".to_string()]
}

fn default_key_visual() -> Vec<String> {
    vec!["v".to_string()]
}

fn default_key_bookmark_list() -> Vec<String> {
    vec!["b".to_string()]
}

fn default_key_bookmark_new() -> Vec<String> {
    vec!["B".to_string()]
}

fn default_key_undo() -> Vec<String> {
    vec!["u".to_string()]
}

fn default_key_quit() -> Vec<String> {
    vec!["q".to_string(), "esc".to_string()]
}
