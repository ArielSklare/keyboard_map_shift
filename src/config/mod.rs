mod io;
mod model;
mod paths;

pub use io::{load_config, save_config};
pub use model::{Config, DEFAULT_HOTKEY_DISPLAY};
pub use paths::config_file_path;
