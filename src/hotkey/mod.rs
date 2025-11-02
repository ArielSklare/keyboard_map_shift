#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedHotkey {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: char,
}

pub fn normalize_display(input: &str) -> String {
    let parts: Vec<&str> = input.split('+').map(|p| p.trim()).collect();
    let normalized: Vec<String> = parts
        .into_iter()
        .map(|p| match p.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => "Ctrl".to_string(),
            "alt" => "Alt".to_string(),
            "shift" => "Shift".to_string(),
            other => {
                if other.len() == 1 {
                    other.to_ascii_uppercase()
                } else {
                    let mut s = other.to_string();
                    if let Some(ch) = s.get_mut(0..1) {
                        let up = ch.to_ascii_uppercase();
                        s.replace_range(0..1, &up);
                    }
                    s
                }
            }
        })
        .collect();
    normalized.join("+")
}

pub fn parse_display(display: &str) -> Result<ParsedHotkey, String> {
    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let mut key: Option<char> = None;

    for part in display.split('+') {
        let p = part.trim().to_ascii_lowercase();
        match p.as_str() {
            "ctrl" | "control" => ctrl = true,
            "alt" => alt = true,
            "shift" => shift = true,
            k if k.len() == 1 => {
                let ch = k.chars().next().unwrap().to_ascii_uppercase();
                key = Some(ch);
            }
            _ => return Err(format!("Unsupported key segment: {}", part)),
        }
    }

    let key = key.ok_or_else(|| "Missing key in hotkey".to_string())?;
    Ok(ParsedHotkey {
        ctrl,
        alt,
        shift,
        key,
    })
}

fn to_gnome_binding(hk: &ParsedHotkey) -> String {
    let mut s = String::new();
    if hk.ctrl {
        s.push_str("<Control>");
    }
    if hk.alt {
        s.push_str("<Alt>");
    }
    if hk.shift {
        s.push_str("<Shift>");
    }
    s.push(hk.key);
    s
}

pub fn to_gnome_binding_from_display(display: &str) -> Result<String, String> {
    let hk = parse_display(display)?;
    Ok(to_gnome_binding(&hk))
}

fn to_windows_hotkey_word(hk: &ParsedHotkey) -> u16 {
    let vk = hk.key as u8;
    let mut mods: u8 = 0;
    if hk.shift {
        mods |= 0x01;
    }
    if hk.ctrl {
        mods |= 0x02;
    }
    if hk.alt {
        mods |= 0x04;
    }
    ((mods as u16) << 8) | (vk as u16)
}

pub fn to_windows_hotkey_word_from_display(display: &str) -> Result<u16, String> {
    let hk = parse_display(display)?;
    Ok(to_windows_hotkey_word(&hk))
}

#[cfg(test)]
mod tests;
