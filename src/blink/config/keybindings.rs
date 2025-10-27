use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Keybindings {}

impl Default for Keybindings {
    fn default() -> Self {
        Self {}
    }
}
