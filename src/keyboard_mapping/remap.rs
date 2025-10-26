use super::types::{KeyboardDirection, KeyboardLayout, LayoutMap};
use std::collections::HashMap;
use unicode_bidi::{BidiClass, bidi_class};

pub fn is_rtl_char(c: char) -> bool {
    matches!(bidi_class(c), BidiClass::R | BidiClass::AL)
}

fn text_starts_rtl(text: &str) -> Option<bool> {
    for ch in text.chars() {
        if ch.is_whitespace() || ch.is_ascii_punctuation() {
            continue;
        }
        return Some(is_rtl_char(ch));
    }
    None
}

fn direction_bonus(text_dir_is_rtl: Option<bool>, layout_dir: KeyboardDirection) -> i32 {
    match (text_dir_is_rtl, layout_dir) {
        (Some(true), KeyboardDirection::RTL) => 5,
        (Some(false), KeyboardDirection::LTR) => 3,
        (Some(true), KeyboardDirection::LTR) => -2,
        (Some(false), KeyboardDirection::RTL) => -2,
        _ => 0,
    }
}

fn coverage_score(text: &str, inverse: &HashMap<char, Vec<u16>>) -> (i32, usize) {
    let mut score: i32 = 0;
    let mut matches: usize = 0;
    for ch in text.chars() {
        if ch.is_control() {
            continue;
        }
        if inverse.contains_key(&ch) {
            score += 2;
            matches += 1;
        } else {
            score -= 1;
        }
    }
    (score, matches)
}

pub fn get_text_leyaout_map<'a>(
    text: &'a str,
    layout_maps: &'a [LayoutMap],
) -> Option<&'a LayoutMap> {
    let text_dir = text_starts_rtl(text);

    // Build all inverse maps once
    let inverses = invert_layout_maps(layout_maps);

    let mut best_idx: Option<usize> = None;
    let mut best_score: i32 = i32::MIN;
    let mut best_matches: usize = 0;

    for (idx, lm) in layout_maps.iter().enumerate() {
        let inverse = &inverses[idx];
        let (mut score, matches) = coverage_score(text, inverse);
        score += direction_bonus(text_dir, lm.layout.direction);

        if score > best_score || (score == best_score && matches > best_matches) {
            best_score = score;
            best_idx = Some(idx);
            best_matches = matches;
        }
    }

    if best_matches == 0 {
        return None;
    }
    best_idx.map(|i| &layout_maps[i])
}

pub fn shift_text_language(
    text: &str,
    curent_layout: &LayoutMap,
    target_layout: &LayoutMap,
) -> String {
    let inverse_current = invert_layout_map(curent_layout);
    text.chars()
        .map(|ch| {
            inverse_current
                .get(&ch)
                .and_then(|vks| vks.first())
                .and_then(|vk| target_layout.map.get(vk))
                .filter(|s| !s.is_empty())
                .cloned()
                .unwrap_or_else(|| ch.to_string())
        })
        .collect()
}

fn invert_layout_map(layout_map: &LayoutMap) -> HashMap<char, Vec<u16>> {
    let mut inverse: HashMap<char, Vec<u16>> = HashMap::new();
    for (vk, output) in &layout_map.map {
        let mut chars = output.chars();
        let first = match chars.next() {
            Some(c) => c,
            None => continue,
        };
        if chars.next().is_none() {
            inverse.entry(first).or_default().push(*vk);
        }
    }
    inverse
}

fn invert_layout_maps(layout_maps: &[LayoutMap]) -> Vec<HashMap<char, Vec<u16>>> {
    layout_maps.iter().map(invert_layout_map).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_layout(
        name: &str,
        direction: KeyboardDirection,
        pairs: Vec<(u16, &str)>,
    ) -> LayoutMap {
        let mut map = HashMap::new();
        for (vk, output) in pairs {
            map.insert(vk, output.to_string());
        }

        LayoutMap {
            layout: KeyboardLayout {
                lang_name: name.to_string(),
                direction,
            },
            map,
        }
    }

    fn create_en_layout() -> LayoutMap {
        create_test_layout(
            "English",
            KeyboardDirection::LTR,
            vec![
                (0x41, "A"),
                (0x42, "B"),
                (0x43, "C"),
                (0x44, "D"),
                (0x45, "E"),
                (0x46, "F"),
                (0x47, "G"),
                (0x48, "H"),
                (0x49, "I"),
                (0x4A, "J"),
                (0x4B, "K"),
                (0x4C, "L"),
                (0x4D, "M"),
                (0x4E, "N"),
                (0x4F, "O"),
                (0x50, "P"),
                (0x51, "Q"),
                (0x52, "R"),
                (0x53, "S"),
                (0x54, "T"),
                (0x55, "U"),
                (0x56, "V"),
                (0x57, "W"),
                (0x58, "X"),
                (0x59, "Y"),
                (0x5A, "Z"),
            ],
        )
    }

    fn create_he_layout() -> LayoutMap {
        create_test_layout(
            "Hebrew",
            KeyboardDirection::RTL,
            vec![
                (0x41, "ש"),
                (0x42, "נ"),
                (0x43, "ב"),
                (0x44, "ג"),
                (0x45, "כ"),
                (0x46, "ע"),
                (0x47, "י"),
                (0x48, "ח"),
                (0x49, "ל"),
                (0x4A, "ך"),
                (0x4B, "פ"),
                (0x4C, "ם"),
                (0x4D, "צ"),
                (0x4E, "ת"),
                (0x4F, "ק"),
                (0x50, "ר"),
                (0x51, "א"),
                (0x52, "ט"),
                (0x53, "ו"),
                (0x54, "ן"),
                (0x55, "מ"),
                (0x56, "ה"),
                (0x57, "ז"),
                (0x58, "ס"),
                (0x59, "ד"),
                (0x5A, "ג"),
            ],
        )
    }

    fn create_ar_layout() -> LayoutMap {
        create_test_layout(
            "Arabic",
            KeyboardDirection::RTL,
            vec![
                (0x41, "ش"),
                (0x42, "س"),
                (0x43, "ي"),
                (0x44, "ب"),
                (0x45, "ل"),
                (0x46, "ا"),
                (0x47, "ت"),
                (0x48, "ن"),
                (0x49, "م"),
                (0x4A, "ك"),
                (0x4B, "ط"),
                (0x4C, "ئ"),
                (0x4D, "ء"),
                (0x4E, "ؤ"),
                (0x4F, "ر"),
                (0x50, "لا"),
                (0x51, "ى"),
                (0x52, "ة"),
                (0x53, "و"),
                (0x54, "ز"),
                (0x55, "ظ"),
                (0x56, "د"),
                (0x57, "ج"),
                (0x58, "ح"),
                (0x59, "خ"),
                (0x5A, "ذ"),
            ],
        )
    }

    #[test]
    fn test_is_rtl_char_arabic() {
        assert!(is_rtl_char('ا'));
        assert!(is_rtl_char('ب'));
        assert!(is_rtl_char('ش'));
    }

    #[test]
    fn test_is_rtl_char_hebrew() {
        assert!(is_rtl_char('א'));
        assert!(is_rtl_char('ב'));
        assert!(is_rtl_char('ש'));
    }

    #[test]
    fn test_is_rtl_char_latin() {
        assert!(!is_rtl_char('A'));
        assert!(!is_rtl_char('a'));
        assert!(!is_rtl_char('Z'));
        assert!(!is_rtl_char('z'));
    }

    #[test]
    fn test_is_rtl_char_numbers_and_symbols() {
        assert!(!is_rtl_char('1'));
        assert!(!is_rtl_char('!'));
        assert!(!is_rtl_char(' '));
        assert!(!is_rtl_char('\n'));
    }

    #[test]
    fn test_text_starts_rtl_arabic() {
        assert_eq!(text_starts_rtl("سلام"), Some(true));
        assert_eq!(text_starts_rtl("مرحبا"), Some(true));
    }

    #[test]
    fn test_text_starts_rtl_hebrew() {
        assert_eq!(text_starts_rtl("שלום"), Some(true));
        assert_eq!(text_starts_rtl("היי"), Some(true));
    }

    #[test]
    fn test_text_starts_rtl_latin() {
        assert_eq!(text_starts_rtl("Hello"), Some(false));
        assert_eq!(text_starts_rtl("world"), Some(false));
    }

    #[test]
    fn test_text_starts_rtl_empty() {
        assert_eq!(text_starts_rtl(""), None);
    }

    #[test]
    fn test_text_starts_rtl_whitespace_only() {
        assert_eq!(text_starts_rtl("   "), None);
        assert_eq!(text_starts_rtl("\n\t"), None);
    }

    #[test]
    fn test_text_starts_rtl_punctuation_only() {
        assert_eq!(text_starts_rtl("!@#$%"), None);
    }

    #[test]
    fn test_text_starts_rtl_mixed_whitespace() {
        assert_eq!(text_starts_rtl("  Hello"), Some(false));
        assert_eq!(text_starts_rtl("  שלום"), Some(true));
    }

    #[test]
    fn test_direction_bonus_rtl_text_rtl_layout() {
        assert_eq!(direction_bonus(Some(true), KeyboardDirection::RTL), 5);
    }

    #[test]
    fn test_direction_bonus_ltr_text_ltr_layout() {
        assert_eq!(direction_bonus(Some(false), KeyboardDirection::LTR), 3);
    }

    #[test]
    fn test_direction_bonus_rtl_text_ltr_layout() {
        assert_eq!(direction_bonus(Some(true), KeyboardDirection::LTR), -2);
    }

    #[test]
    fn test_direction_bonus_ltr_text_rtl_layout() {
        assert_eq!(direction_bonus(Some(false), KeyboardDirection::RTL), -2);
    }

    #[test]
    fn test_direction_bonus_no_text_direction() {
        assert_eq!(direction_bonus(None, KeyboardDirection::LTR), 0);
        assert_eq!(direction_bonus(None, KeyboardDirection::RTL), 0);
    }

    #[test]
    fn test_coverage_score_perfect_match() {
        let mut inverse = HashMap::new();
        inverse.insert('A', vec![0x41]);
        inverse.insert('B', vec![0x42]);

        let (score, matches) = coverage_score("AB", &inverse);
        assert_eq!(matches, 2);
        assert_eq!(score, 4);
    }

    #[test]
    fn test_coverage_score_partial_match() {
        let mut inverse = HashMap::new();
        inverse.insert('A', vec![0x41]);

        let (score, matches) = coverage_score("ABC", &inverse);
        assert_eq!(matches, 1);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_coverage_score_no_match() {
        let inverse = HashMap::new();

        let (score, matches) = coverage_score("ABC", &inverse);
        assert_eq!(matches, 0);
        assert_eq!(score, -3);
    }

    #[test]
    fn test_coverage_score_ignores_control_chars() {
        let mut inverse = HashMap::new();
        inverse.insert('A', vec![0x41]);

        let (score, matches) = coverage_score("A\n\t", &inverse);
        assert_eq!(matches, 1);
        assert_eq!(score, 2);
    }

    #[test]
    fn test_get_text_layout_map_single_match() {
        let layouts = vec![create_en_layout()];
        let result = get_text_leyaout_map("HELLO", &layouts);

        assert!(result.is_some());
        assert_eq!(result.unwrap().layout.lang_name, "English");
    }

    #[test]
    fn test_get_text_layout_map_multiple_layouts_best_match() {
        let layouts = vec![create_en_layout(), create_he_layout()];
        let result = get_text_leyaout_map("HELLO", &layouts);

        assert!(result.is_some());
        assert_eq!(result.unwrap().layout.lang_name, "English");
    }

    #[test]
    fn test_get_text_layout_map_rtl_preference() {
        let layouts = vec![create_en_layout(), create_he_layout()];
        let result = get_text_leyaout_map("שלום", &layouts);

        assert!(result.is_some());
        assert_eq!(result.unwrap().layout.lang_name, "Hebrew");
    }

    #[test]
    fn test_get_text_layout_map_no_match() {
        let layouts = vec![create_en_layout()];
        let result = get_text_leyaout_map("中文", &layouts);

        assert!(result.is_none());
    }

    #[test]
    fn test_get_text_layout_map_empty_text() {
        let layouts = vec![create_en_layout()];
        let result = get_text_leyaout_map("", &layouts);

        assert!(result.is_none());
    }

    #[test]
    fn test_shift_text_language_full_match() {
        let current = create_en_layout();
        let target = create_he_layout();

        let result = shift_text_language("A", &current, &target);
        assert_eq!(result, "ש");
    }

    #[test]
    fn test_shift_text_language_partial_match() {
        let current = create_en_layout();
        let target = create_he_layout();

        let result = shift_text_language("AZ中", &current, &target);
        assert_eq!(result, "שג中");
    }

    #[test]
    fn test_shift_text_language_no_match() {
        let current = create_en_layout();
        let target = create_he_layout();

        let result = shift_text_language("123", &current, &target);
        assert_eq!(result, "123");
    }

    #[test]
    fn test_shift_text_language_multi_char_sequences() {
        let mut current_map = HashMap::new();
        current_map.insert(0x50, "لا".to_string()); // Arabic la

        let mut target_map = HashMap::new();
        target_map.insert(0x50, "LA".to_string()); // English LA

        let current = LayoutMap {
            layout: KeyboardLayout {
                lang_name: "Arabic".to_string(),
                direction: KeyboardDirection::RTL,
            },
            map: current_map,
        };

        let target = LayoutMap {
            layout: KeyboardLayout {
                lang_name: "English".to_string(),
                direction: KeyboardDirection::LTR,
            },
            map: target_map,
        };

        let result = shift_text_language("لا", &current, &target);
        assert_eq!(result, "لا");
    }

    #[test]
    fn test_invert_layout_map_single_chars() {
        let layout = create_en_layout();
        let inverse = invert_layout_map(&layout);

        assert_eq!(inverse.get(&'A'), Some(&vec![0x41]));
        assert_eq!(inverse.get(&'B'), Some(&vec![0x42]));
    }

    #[test]
    fn test_invert_layout_map_ignores_multi_char() {
        let mut map = HashMap::new();
        map.insert(0x41, "AB".to_string());
        map.insert(0x42, "C".to_string());

        let layout = LayoutMap {
            layout: KeyboardLayout {
                lang_name: "Test".to_string(),
                direction: KeyboardDirection::LTR,
            },
            map,
        };

        let inverse = invert_layout_map(&layout);
        assert!(inverse.get(&'A').is_none());
        assert_eq!(inverse.get(&'C'), Some(&vec![0x42]));
    }

    #[test]
    fn test_invert_layout_map_empty_strings() {
        let mut map = HashMap::new();
        map.insert(0x41, "".to_string());
        map.insert(0x42, "A".to_string());

        let layout = LayoutMap {
            layout: KeyboardLayout {
                lang_name: "Test".to_string(),
                direction: KeyboardDirection::LTR,
            },
            map,
        };

        let inverse = invert_layout_map(&layout);
        assert_eq!(inverse.get(&'A'), Some(&vec![0x42]));
    }

    #[test]
    fn test_invert_layout_maps_batch() {
        let layouts = vec![create_en_layout(), create_he_layout()];
        let inverses = invert_layout_maps(&layouts);

        assert_eq!(inverses.len(), 2);
        assert_eq!(inverses[0].get(&'A'), Some(&vec![0x41]));
        assert_eq!(inverses[1].get(&'ש'), Some(&vec![0x41]));
    }
}
