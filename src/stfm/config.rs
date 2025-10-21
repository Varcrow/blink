use color_eyre::eyre::Ok;
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
    pub keybindings: Keybindings,
    #[serde(default)]
    pub behavior: Behavior,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UiConfig::default(),
            keybindings: Keybindings::default(),
            behavior: Behavior::default(),
        }
    }
}

impl Config {
    pub fn load() -> color_eyre::Result<Self> {
        let config_path = Self::config_path()?;

        // create default config if not found
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
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_hidden: default_show_hidden(),
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
