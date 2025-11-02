use serde::{Deserialize, Serialize};

pub const DEFAULT_HOTKEY_DISPLAY: &str = "Ctrl+Alt+K";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub hotkey: String,
}

impl Config {
    pub fn with_defaults() -> Self {
        Self {
            hotkey: DEFAULT_HOTKEY_DISPLAY.to_string(),
        }
    }
}

#[cfg(test)]
mod tests;
