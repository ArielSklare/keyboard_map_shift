#![cfg(not(any(target_os = "windows", target_os = "linux")))]

pub fn get_highlighted_text() -> Option<String> {
    panic!("get_highlighted: get_highlighted_text is not implemented for this OS");
}

pub fn replace_highlighted_text(_new_text: &str) -> Result<(), String> {
    panic!("get_highlighted: replace_highlighted_text is not implemented for this OS");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(
        expected = "get_highlighted: get_highlighted_text is not implemented for this OS"
    )]
    fn test_get_highlighted_text_panics() {
        get_highlighted_text();
    }

    #[test]
    #[should_panic(
        expected = "get_highlighted: replace_highlighted_text is not implemented for this OS"
    )]
    fn test_replace_highlighted_text_panics() {
        replace_highlighted_text("test");
    }
}
