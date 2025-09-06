use std::collections::HashMap;

// ===== Types (shared) =====
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardLayout {
    pub lang_name: String,
    pub direction: KeyboardDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardDirection {
    LTR,
    RTL,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMap {
    pub layout: KeyboardLayout,
    pub map: HashMap<u16, String>,
}
