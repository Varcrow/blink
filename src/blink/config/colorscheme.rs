use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorScheme {
    // Directory colors
    #[serde(default = "default_directory")]
    pub directory: ColorConfig,
    #[serde(default = "default_directory_selected")]
    pub directory_selected: ColorConfig,

    // File type colors
    #[serde(default = "default_file_default")]
    pub file_default: ColorConfig,
    #[serde(default = "default_file_executable")]
    pub file_executable: ColorConfig,
    #[serde(default = "default_file_archive")]
    pub file_archive: ColorConfig,
    #[serde(default = "default_file_image")]
    pub file_image: ColorConfig,
    #[serde(default = "default_file_video")]
    pub file_video: ColorConfig,
    #[serde(default = "default_file_audio")]
    pub file_audio: ColorConfig,
    #[serde(default = "default_file_document")]
    pub file_document: ColorConfig,

    // Programming language colors
    #[serde(default = "default_lang_rust")]
    pub lang_rust: ColorConfig,
    #[serde(default = "default_lang_python")]
    pub lang_python: ColorConfig,
    #[serde(default = "default_lang_javascript")]
    pub lang_javascript: ColorConfig,
    #[serde(default = "default_lang_typescript")]
    pub lang_typescript: ColorConfig,
    #[serde(default = "default_lang_c")]
    pub lang_c: ColorConfig,
    #[serde(default = "default_lang_cpp")]
    pub lang_cpp: ColorConfig,
    #[serde(default = "default_lang_java")]
    pub lang_java: ColorConfig,
    #[serde(default = "default_lang_go")]
    pub lang_go: ColorConfig,

    // UI colors
    #[serde(default = "default_selected_bg")]
    pub selected_bg: ColorConfig,
    #[serde(default = "default_visual_selection_bg")]
    pub visual_selection_bg: ColorConfig,
    #[serde(default = "default_border")]
    pub border: ColorConfig,
    #[serde(default = "default_status_bar")]
    pub status_bar: ColorConfig,
    #[serde(default = "default_prompt_bg")]
    pub prompt_bg: ColorConfig,
    #[serde(default = "default_prompt_border")]
    pub prompt_border: ColorConfig,

    // Log colors
    #[serde(default = "default_log_info")]
    pub log_info: ColorConfig,
    #[serde(default = "default_log_warning")]
    pub log_warning: ColorConfig,
    #[serde(default = "default_log_error")]
    pub log_error: ColorConfig,
}

// Default functions for directories
fn default_directory() -> ColorConfig {
    ColorConfig::Rgb { r: 100, g: 181, b: 246 }
}

fn default_directory_selected() -> ColorConfig {
    ColorConfig::Rgb { r: 66, g: 165, b: 245 }
}

// Default functions for file types
fn default_file_default() -> ColorConfig {
    ColorConfig::Rgb { r: 189, g: 189, b: 189 }
}

fn default_file_executable() -> ColorConfig {
    ColorConfig::Rgb { r: 76, g: 175, b: 80 }
}

fn default_file_archive() -> ColorConfig {
    ColorConfig::Rgb { r: 239, g: 83, b: 80 }
}

fn default_file_image() -> ColorConfig {
    ColorConfig::Rgb { r: 171, g: 71, b: 188 }
}

fn default_file_video() -> ColorConfig {
    ColorConfig::Rgb { r: 255, g: 82, b: 82 }
}

fn default_file_audio() -> ColorConfig {
    ColorConfig::Rgb { r: 156, g: 39, b: 176 }
}

fn default_file_document() -> ColorConfig {
    ColorConfig::Rgb { r: 33, g: 150, b: 243 }
}

// Default functions for programming languages
fn default_lang_rust() -> ColorConfig {
    ColorConfig::Rgb { r: 222, g: 165, b: 132 }
}

fn default_lang_python() -> ColorConfig {
    ColorConfig::Rgb { r: 53, g: 114, b: 165 }
}

fn default_lang_javascript() -> ColorConfig {
    ColorConfig::Rgb { r: 240, g: 219, b: 79 }
}

fn default_lang_typescript() -> ColorConfig {
    ColorConfig::Rgb { r: 49, g: 120, b: 198 }
}

fn default_lang_c() -> ColorConfig {
    ColorConfig::Rgb { r: 85, g: 85, b: 85 }
}

fn default_lang_cpp() -> ColorConfig {
    ColorConfig::Rgb { r: 0, g: 89, b: 157 }
}

fn default_lang_java() -> ColorConfig {
    ColorConfig::Rgb { r: 244, g: 67, b: 54 }
}

fn default_lang_go() -> ColorConfig {
    ColorConfig::Rgb { r: 0, g: 173, b: 216 }
}

// Default functions for UI colors
fn default_selected_bg() -> ColorConfig {
    ColorConfig::Named("darkgray".to_string())
}

fn default_visual_selection_bg() -> ColorConfig {
    ColorConfig::Rgb { r: 80, g: 80, b: 120 }
}

fn default_border() -> ColorConfig {
    ColorConfig::Named("white".to_string())
}

fn default_status_bar() -> ColorConfig {
    ColorConfig::Named("white".to_string())
}

fn default_prompt_bg() -> ColorConfig {
    ColorConfig::Named("black".to_string())
}

fn default_prompt_border() -> ColorConfig {
    ColorConfig::Named("white".to_string())
}

// Default functions for logs
fn default_log_info() -> ColorConfig {
    ColorConfig::Named("cyan".to_string())
}
fn default_log_warning() -> ColorConfig {
    ColorConfig::Named("yellow".to_string())
}
fn default_log_error() -> ColorConfig {
    ColorConfig::Named("red".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ColorConfig {
    Named(String), // "red", "blue", etc.
    Rgb { r: u8, g: u8, b: u8 },
}

impl ColorConfig {
    pub fn to_ratatui_color(&self) -> Color {
        match self {
            ColorConfig::Named(name) => match name.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" => Color::Magenta,
                "cyan" => Color::Cyan,
                "gray" | "grey" => Color::Gray,
                "darkgray" | "darkgrey" => Color::DarkGray,
                "lightred" => Color::LightRed,
                "lightgreen" => Color::LightGreen,
                "lightyellow" => Color::LightYellow,
                "lightblue" => Color::LightBlue,
                "lightmagenta" => Color::LightMagenta,
                "lightcyan" => Color::LightCyan,
                "white" => Color::White,
                _ => Color::White,
            },
            ColorConfig::Rgb { r, g, b } => Color::Rgb(*r, *g, *b),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            directory: default_directory(),
            directory_selected: default_directory_selected(),
            file_default: default_file_default(),
            file_executable: default_file_executable(),
            file_archive: default_file_archive(),
            file_image: default_file_image(),
            file_video: default_file_video(),
            file_audio: default_file_audio(),
            file_document: default_file_document(),
            lang_rust: default_lang_rust(),
            lang_python: default_lang_python(),
            lang_javascript: default_lang_javascript(),
            lang_typescript: default_lang_typescript(),
            lang_c: default_lang_c(),
            lang_cpp: default_lang_cpp(),
            lang_java: default_lang_java(),
            lang_go: default_lang_go(),
            selected_bg: default_selected_bg(),
            visual_selection_bg: default_visual_selection_bg(),
            border: default_border(),
            status_bar: default_status_bar(),
            prompt_bg: default_prompt_bg(),
            prompt_border: default_prompt_border(),
            log_info: default_log_info(),
            log_warning: default_log_warning(),
            log_error: default_log_error(),
        }
    }
}
