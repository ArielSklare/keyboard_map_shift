use super::*;

#[test]
#[should_panic(expected = "get_highlighted: get_highlighted_text is not implemented for this OS")]
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
