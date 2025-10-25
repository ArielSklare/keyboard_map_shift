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
    let mut result: String = text
        .chars()
        .map(|ch| {
            inverse_current
                .get(&ch)
                .and_then(|vks| vks.first())
                .and_then(|vk| target_layout.map.get(vk))
                .filter(|s| !s.is_empty())
                .cloned()
                .unwrap_or_else(|| ch.to_string())
        })
        .collect();

    // Check if layouts have different directions and reverse text if needed
    if curent_layout.layout.direction != target_layout.layout.direction {
        result = reverse_text_direction(&result);
    }

    result
}

/// Reverse the text direction for RTL/LTR conversion
/// This function reverses the order of characters while preserving the logical structure
fn reverse_text_direction(text: &str) -> String {
    text.chars().rev().collect()
}

/// Build a reverse mapping for a single layout: char -> list of virtual keys (u16)
/// Only single-character outputs are included; multi-character outputs are skipped.
fn invert_layout_map(layout_map: &LayoutMap) -> HashMap<char, Vec<u16>> {
    let mut inverse: HashMap<char, Vec<u16>> = HashMap::new();
    for (vk, output) in &layout_map.map {
        let mut chars = output.chars();
        let first = match chars.next() {
            Some(c) => c,
            None => continue,
        };
        // Include only single-character outputs to avoid ambiguous multi-char sequences
        if chars.next().is_none() {
            inverse.entry(first).or_default().push(*vk);
        }
    }
    inverse
}

/// Build reverse mappings for all layouts, preserving order with the input slice.
fn invert_layout_maps(layout_maps: &[LayoutMap]) -> Vec<HashMap<char, Vec<u16>>> {
    layout_maps.iter().map(invert_layout_map).collect()
}
