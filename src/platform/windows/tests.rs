use super::*;
use crate::test_utils::{EnvVarGuard, TempDirGuard};

fn temp_appdata() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let unique = format!(
        "kms_test_appdata_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    p.push(unique);
    let _ = std::fs::create_dir_all(&p);
    p
}

struct AppDataGuard {
    _guard: EnvVarGuard,
}
impl AppDataGuard {
    fn set(path: &std::path::Path) -> Self {
        AppDataGuard {
            _guard: EnvVarGuard::set_path(WINDOWS_PATHS.env_appdata, path),
        }
    }
}

#[test]
fn windows_binder_smoke() {
    let appdata = temp_appdata();
    let _dir = TempDirGuard::new(appdata.clone());
    let _guard = AppDataGuard::set(&appdata);

    let binder = WindowsBinder::new();
    let _ = binder.apply_hotkey("Ctrl+Alt+K");
    let _ = binder.remove_hotkey();
}
