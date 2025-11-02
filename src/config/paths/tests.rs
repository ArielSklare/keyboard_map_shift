use super::*;

#[test]
fn config_file_path_smoke_and_filename() {
    let path = config_file_path().unwrap();
    assert_eq!(path.file_name().unwrap().to_string_lossy(), "config.toml");
}
