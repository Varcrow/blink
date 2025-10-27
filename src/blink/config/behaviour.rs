use serde::{Deserialize, Serialize};

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

fn default_confirm_delete() -> bool {
    true
}
