#![cfg(target_os = "windows")]

use std::{mem, ptr, thread, time::Duration};
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    System::{
        Com::{
            CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
            CoUninitialize,
        },
        DataExchange::{
            CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData,
        },
        Memory::{GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalUnlock},
    },
    UI::{
        Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationTextPattern, IUIAutomationValuePattern,
            UIA_TextPatternId, UIA_ValuePatternId,
        },
        Input::KeyboardAndMouse::{
            INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP,
            SendInput, VK_C, VK_CONTROL,
        },
    },
};
// Using GetCurrentPatternAs<T>(), no need to import Interface::IID

// Clipboard format for UTF-16 text
const CF_UNICODETEXT: u32 = 13;

fn read_clipboard_unicode() -> Option<String> {
    unsafe {
        if OpenClipboard(None).is_ok() {
            let mut result: Option<String> = None;
            if let Ok(h) = GetClipboardData(CF_UNICODETEXT) {
                if !h.0.is_null() {
                    let ptr_wide = GlobalLock(HGLOBAL(h.0)) as *const u16;
                    if !ptr_wide.is_null() {
                        let mut len = 0usize;
                        while *ptr_wide.add(len) != 0 {
                            len += 1;
                        }
                        let slice = std::slice::from_raw_parts(ptr_wide, len);
                        result = Some(String::from_utf16_lossy(slice));
                        let _ = GlobalUnlock(HGLOBAL(h.0));
                    }
                }
            }
            let _ = CloseClipboard();
            return result;
        }
    }
    None
}

fn write_clipboard_unicode(text: &str) -> bool {
    unsafe {
        if OpenClipboard(None).is_err() {
            return false;
        }
        if EmptyClipboard().is_err() {
            let _ = CloseClipboard();
            return false;
        }

        let utf16: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes_len = utf16.len() * mem::size_of::<u16>();
        let hmem = match GlobalAlloc(GMEM_MOVEABLE, bytes_len) {
            Ok(h) => h,
            Err(_) => {
                let _ = CloseClipboard();
                return false;
            }
        };
        let dest = GlobalLock(hmem) as *mut u8;
        if dest.is_null() {
            let _ = CloseClipboard();
            return false;
        }
        ptr::copy_nonoverlapping(utf16.as_ptr() as *const u8, dest, bytes_len);
        let _ = GlobalUnlock(hmem);

        let handle = HANDLE(hmem.0);
        let set_ok = SetClipboardData(CF_UNICODETEXT, Some(handle)).is_ok();
        let _ = CloseClipboard();
        set_ok
    }
}

fn send_ctrl_c() {
    unsafe {
        let inputs: [INPUT; 4] = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];
        let _ = SendInput(&inputs, mem::size_of::<INPUT>() as i32);
    }
}

pub fn get_highlighted_text() -> Option<String> {
    // First try UI Automation to read selection/text directly
    if let Some(via_uia) = try_uia_get_selection_text() {
        if !via_uia.is_empty() {
            return Some(via_uia);
        }
    }

    // Fallback: Save clipboard, Ctrl+C, read, restore
    copy_read_restore_clipboard()
}

fn copy_read_restore_clipboard() -> Option<String> {
    let old = read_clipboard_unicode();
    send_ctrl_c();
    // Small delay for target app to process
    thread::sleep(Duration::from_millis(60));
    let new_val = read_clipboard_unicode();
    match &old {
        Some(s) => {
            let _ = write_clipboard_unicode(s);
        }
        None => unsafe {
            if OpenClipboard(None).is_ok() {
                let _ = EmptyClipboard();
                let _ = CloseClipboard();
            }
        },
    }
    new_val
}

fn try_uia_get_selection_text() -> Option<String> {
    unsafe {
        if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_err() {
            return None;
        }
        let _guard = CoUninitGuard;

        let automation: IUIAutomation =
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;
        let focused = automation.GetFocusedElement().ok()?;

        // Try TextPattern first
        if let Ok(text_pat) =
            focused.GetCurrentPatternAs::<IUIAutomationTextPattern>(UIA_TextPatternId)
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
            focused.GetCurrentPatternAs::<IUIAutomationValuePattern>(UIA_ValuePatternId)
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

struct CoUninitGuard;
impl Drop for CoUninitGuard {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
