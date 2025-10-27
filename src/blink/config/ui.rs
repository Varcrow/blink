use ratatui::widgets::BorderType;
use serde::{Deserialize, Serialize};

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

impl UiConfig {
    // convert string to border type
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

fn default_show_hidden() -> bool {
    true
}
fn default_border_type() -> String {
    "plain".to_string()
}
