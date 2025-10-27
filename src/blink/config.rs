use color_eyre::eyre::Ok;
use ratatui::style::Color;
use ratatui::widgets::BorderType;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/*
 * Notes for meself to remember
 * ----------------------------
 * 1. #[serde(default = "default_show_hidden")] is for when a certain field is missing from a
 *    declared config file section e.g. paste keybinding missing from [ui]. This makes it so we don't
 *    get errors if unless we declare every option.
 *
 * 2. We still implement default for each struct as a fall back for when an entire section isnt
 *    declared e.g. [keybindings] exist in the toml but [behaviour] doesn't, therefor use thy
 *    defaults :D
 *
 * 3. why does config go into roaming on windows lol
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub colors: ColorScheme,
    #[serde(default)]
    pub keybindings: Keybindings,
    #[serde(default)]
    pub behavior: Behavior,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UiConfig::default(),
            colors: ColorScheme::default(),
            keybindings: Keybindings::default(),
            behavior: Behavior::default(),
        }
    }
}

impl Config {
    pub fn load() -> color_eyre::Result<Self> {
        let config_path = Self::config_path()?;

        // Create default config if not found
        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let contents = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    fn save(&self) -> color_eyre::Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string_pretty(self)?;
        fs::write(&config_path, toml_string)?;

        Ok(())
    }

    // finds the proper config path depending on os
    fn config_path() -> color_eyre::Result<PathBuf> {
        use directories::ProjectDirs;

        let proj_dirs = ProjectDirs::from("com", "Varcrow", "blink")
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not determine config directory"))?;

        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("blink.toml"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_show_hidden")]
    pub show_hidden: bool,
    #[serde(default = "default_border_type")]
    pub border_type: String,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_hidden: default_show_hidden(),
            border_type: default_border_type(),
        }
    }
}

// Helper to convert string to BorderType
impl UiConfig {
    pub fn get_border_type(&self) -> BorderType {
        match self.border_type.to_lowercase().as_str() {
            "plain" => BorderType::Plain,
            "rounded" => BorderType::Rounded,
            "double" => BorderType::Double,
            "thick" => BorderType::Thick,
            "quadrantinside" => BorderType::QuadrantInside,
            "quadrantoutside" => BorderType::QuadrantOutside,
            _ => BorderType::Rounded, // Default fallback
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Keybindings {}

impl Default for Keybindings {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Behavior {
    #[serde(default = "default_confirm_delete")]
    pub confirm_delete: bool,
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            confirm_delete: default_confirm_delete(),
        }
    }
}

//default functions
fn default_show_hidden() -> bool {
    true
}

fn default_confirm_delete() -> bool {
    true
}

fn default_border_type() -> String {
    "plain".to_string()
}
