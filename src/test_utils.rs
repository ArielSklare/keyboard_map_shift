use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

pub struct EnvVarGuard {
    key: &'static str,
    old: Option<OsString>,
}

impl EnvVarGuard {
    pub fn set_str(key: &'static str, value: &str) -> Self {
        let old = std::env::var_os(key);
        unsafe {
            std::env::set_var(key, value);
        }
        Self { key, old }
    }

    pub fn set_os(key: &'static str, value: &OsStr) -> Self {
        let old = std::env::var_os(key);
        unsafe {
            std::env::set_var(key, value);
        }
        Self { key, old }
    }

    pub fn set_path(key: &'static str, path: &Path) -> Self {
        Self::set_os(key, path.as_os_str())
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match self.old.take() {
            Some(v) => unsafe {
                std::env::set_var(self.key, v);
            },
            None => unsafe {
                std::env::remove_var(self.key);
            },
        }
    }
}

pub struct TempDirGuard {
    path: PathBuf,
}

impl TempDirGuard {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
