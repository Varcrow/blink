use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorScheme {
    // Directory colors
    pub directory: ColorConfig,
    pub directory_selected: ColorConfig,

    // File type colors
    pub file_default: ColorConfig,
    pub file_executable: ColorConfig,
    pub file_archive: ColorConfig,
    pub file_image: ColorConfig,
    pub file_video: ColorConfig,
    pub file_audio: ColorConfig,
    pub file_document: ColorConfig,

    // Programming language colors
    pub lang_rust: ColorConfig,
    pub lang_python: ColorConfig,
    pub lang_javascript: ColorConfig,
    pub lang_typescript: ColorConfig,
    pub lang_c: ColorConfig,
    pub lang_cpp: ColorConfig,
    pub lang_java: ColorConfig,
    pub lang_go: ColorConfig,

    // UI colors
    pub selected_bg: ColorConfig,
    pub visual_selection_bg: ColorConfig,
    pub border: ColorConfig,
    pub status_bar: ColorConfig,
    pub prompt_bg: ColorConfig,
    pub prompt_border: ColorConfig,
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
                _ => Color::White, // Default fallback
            },
            ColorConfig::Rgb { r, g, b } => Color::Rgb(*r, *g, *b),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            // Directories
            directory: ColorConfig::Rgb {
                r: 100,
                g: 181,
                b: 246,
            },
            directory_selected: ColorConfig::Rgb {
                r: 66,
                g: 165,
                b: 245,
            },

            // File types
            file_default: ColorConfig::Rgb {
                r: 189,
                g: 189,
                b: 189,
            },
            file_executable: ColorConfig::Rgb {
                r: 76,
                g: 175,
                b: 80,
            },
            file_archive: ColorConfig::Rgb {
                r: 239,
                g: 83,
                b: 80,
            },
            file_image: ColorConfig::Rgb {
                r: 171,
                g: 71,
                b: 188,
            },
            file_video: ColorConfig::Rgb {
                r: 255,
                g: 82,
                b: 82,
            },
            file_audio: ColorConfig::Rgb {
                r: 156,
                g: 39,
                b: 176,
            },
            file_document: ColorConfig::Rgb {
                r: 33,
                g: 150,
                b: 243,
            },

            // Languages
            lang_rust: ColorConfig::Rgb {
                r: 222,
                g: 165,
                b: 132,
            },
            lang_python: ColorConfig::Rgb {
                r: 53,
                g: 114,
                b: 165,
            },
            lang_javascript: ColorConfig::Rgb {
                r: 240,
                g: 219,
                b: 79,
            },
            lang_typescript: ColorConfig::Rgb {
                r: 49,
                g: 120,
                b: 198,
            },
            lang_c: ColorConfig::Rgb {
                r: 85,
                g: 85,
                b: 85,
            },
            lang_cpp: ColorConfig::Rgb {
                r: 0,
                g: 89,
                b: 157,
            },
            lang_java: ColorConfig::Rgb {
                r: 244,
                g: 67,
                b: 54,
            },
            lang_go: ColorConfig::Rgb {
                r: 0,
                g: 173,
                b: 216,
            },

            // UI
            selected_bg: ColorConfig::Named("darkgray".to_string()),
            visual_selection_bg: ColorConfig::Rgb {
                r: 80,
                g: 80,
                b: 120,
            },
            border: ColorConfig::Named("white".to_string()),
            status_bar: ColorConfig::Named("white".to_string()),
            prompt_bg: ColorConfig::Named("black".to_string()),
            prompt_border: ColorConfig::Named("white".to_string()),
        }
    }
}
