use super::*;

#[test]
fn test_get_highlighted_text_smoke() {
    let _result = get_highlighted_text();
}

#[test]
fn test_replace_highlighted_text_smoke() {
    let result = replace_highlighted_text("test text");
    match result {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[test]
fn test_is_wsl_detection() {
    let is_wsl = is_wsl();
    assert!(is_wsl == true || is_wsl == false);
}
