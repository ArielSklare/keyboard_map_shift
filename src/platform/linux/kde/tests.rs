use super::*;
use crate::test_utils::{EnvVarGuard, TempDirGuard};

fn temp_home() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let unique = format!(
        "kms_test_home_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    p.push(unique);
    let _ = std::fs::create_dir_all(&p);
    p
}

#[allow(dead_code)]
struct HomeGuard {
    _guard: EnvVarGuard,
}
impl HomeGuard {
    fn set(path: &std::path::Path) -> Self {
        HomeGuard {
            _guard: EnvVarGuard::set_path("HOME", path),
        }
    }
}

#[test]
fn apply_kde_binding_smoke_with_temp_home() {
    let home = temp_home();
    let _home_dir = TempDirGuard::new(home.clone());
    let _home_guard = HomeGuard::set(&home);
    let _ = apply_kde_binding("Ctrl+Alt+K");
}
