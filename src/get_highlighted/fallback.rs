#![cfg(not(any(target_os = "windows", target_os = "linux")))]

pub fn get_highlighted_text() -> Option<String> {
    panic!("get_highlighted: get_highlighted_text is not implemented for this OS");
}
