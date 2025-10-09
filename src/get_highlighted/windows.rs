#![cfg(target_os = "windows")]

use std::mem;
use windows::Win32::{
    System::Com::{
        CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
        CoUninitialize,
    },
    UI::{
        Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationTextPattern,
            IUIAutomationValuePattern, TreeScope_Children, TreeScope_Descendants,
            UIA_TextPatternId, UIA_ValuePatternId,
        },
        Input::KeyboardAndMouse::{
            INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP,
            SendInput, VIRTUAL_KEY,
        },
    },
};
// Using GetCurrentPatternAs<T>(), no need to import Interface::IID

fn type_unicode_text(text: &str) {
    unsafe {
        // Each UTF-16 code unit is sent as a KEYDOWN with UNICODE flag, then KEYUP
        let utf16: Vec<u16> = text.encode_utf16().collect();
        // Build inputs: two per code unit (down + up)
        let mut inputs: Vec<INPUT> = Vec::with_capacity(utf16.len() * 2);
        for unit in utf16 {
            // Key down
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0), // VK is 0 for unicode events
                        wScan: unit,
                        dwFlags: KEYBD_EVENT_FLAGS(0x0004), // KEYEVENTF_UNICODE
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
            // Key up
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: unit,
                        dwFlags: KEYBD_EVENT_FLAGS(0x0004 | 0x0002), // UNICODE | KEYUP
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
        }
        if !inputs.is_empty() {
            let _ = SendInput(&inputs, mem::size_of::<INPUT>() as i32);
        }
    }
}

pub fn get_highlighted_text() -> Option<String> {
    try_uia_get_selection_text()
}

fn try_uia_get_selection_text() -> Option<String> {
    unsafe {
        if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_err() {
            return None;
        }
        let _guard = CoUninitGuard;

        let automation: IUIAutomation =
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;

        // Try multiple approaches to get selected text

        // 1. Try focused element first
        if let Ok(focused) = automation.GetFocusedElement() {
            if let Some(text) = try_get_text_from_element(&focused) {
                return Some(text);
            }
        }

        // 2. Try to find text elements with selections in the entire desktop
        if let Ok(desktop) = automation.GetRootElement() {
            if let Some(text) = find_selected_text_in_tree(&automation, &desktop) {
                return Some(text);
            }
        }

        None
    }
}

fn try_get_text_from_element(element: &IUIAutomationElement) -> Option<String> {
    unsafe {
        // Try TextPattern first (for text selections)
        if let Ok(text_pat) =
            element.GetCurrentPatternAs::<IUIAutomationTextPattern>(UIA_TextPatternId)
        {
            if let Ok(sel_array) = text_pat.GetSelection() {
                let len = sel_array.Length().unwrap_or(0);
                let mut collected = String::new();
                for i in 0..len {
                    if let Ok(range) = sel_array.GetElement(i) {
                        if let Ok(s) = range.GetText(i32::MAX) {
                            let chunk = s.to_string();
                            if !chunk.is_empty() {
                                collected.push_str(&chunk);
                            }
                        }
                    }
                }
                if !collected.is_empty() {
                    return Some(collected);
                }
            }
        }

        // Fall back to ValuePattern (single-line edits often expose this)
        if let Ok(val_pat) =
            element.GetCurrentPatternAs::<IUIAutomationValuePattern>(UIA_ValuePatternId)
        {
            if let Ok(s) = val_pat.CurrentValue() {
                let v = s.to_string();
                if !v.is_empty() {
                    return Some(v);
                }
            }
        }

        None
    }
}

fn find_selected_text_in_tree(
    automation: &IUIAutomation,
    root: &IUIAutomationElement,
) -> Option<String> {
    unsafe {
        // Recursively search through the element tree for text selections
        if let Some(text) = try_get_text_from_element(root) {
            return Some(text);
        }

        // Search children
        if let Ok(children) = root.FindAll(
            TreeScope_Children,
            &automation.CreateTrueCondition().unwrap(),
        ) {
            let count = children.Length().unwrap_or(0);
            for i in 0..count {
                if let Ok(child) = children.GetElement(i) {
                    if let Some(text) = find_selected_text_in_tree(automation, &child) {
                        return Some(text);
                    }
                }
            }
        }

        None
    }
}

struct CoUninitGuard;
impl Drop for CoUninitGuard {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

pub fn replace_highlighted_text(new_text: &str) -> Result<(), String> {
    // Type Unicode characters directly so the current selection is replaced without touching the clipboard
    type_unicode_text(new_text);
    Ok(())
}
