//! Input Simulation
//!
//! Uses SendInput to send keystrokes (backspace and Unicode characters).

use tracing::debug;

#[cfg(windows)]
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VK_BACK,
};

/// Extra info flag to identify our own injected keys
pub const BUTTRE_INJECTED: usize = 0x564B4559; // "VKEY" in hex

/// Send backspace keys (optimized with batching)
#[cfg(windows)]
pub fn send_backspaces(count: usize) {
    if count == 0 {
        return;
    }

    debug!("Sending {} backspaces", count);

    // Batch backspaces for better performance
    let mut inputs = Vec::with_capacity(count * 2);

    for _ in 0..count {
        // Key down
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_BACK as u16,
                    wScan: 0,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: BUTTRE_INJECTED,
                },
            },
        });

        // Key up
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_BACK as u16,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: BUTTRE_INJECTED,
                },
            },
        });
    }

    // Send all at once with error checking
    // SAFETY:
    // 1. inputs is a valid Vec<INPUT> allocated on the stack
    // 2. as_mut_ptr() returns valid pointer to first element (or null if empty, but we check count == 0 above)
    // 3. expected count matches actual Vec length
    // 4. size_of::<INPUT>() is the correct structure size for SendInput
    // 5. SendInput is properly declared in windows_sys
    // 6. INPUT structs are properly initialized with valid VK codes and flags
    // 7. All memory is valid for the duration of the SendInput call
    unsafe {
        let expected = inputs.len() as u32;
        let sent = SendInput(
            expected,
            inputs.as_mut_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );

        if sent != expected {
            tracing::error!("SendInput failed: expected {}, sent {}", expected, sent);
        }
    }
}

#[cfg(not(windows))]
pub fn send_backspaces(_count: usize) {}

/// Send a string as Unicode characters (optimized with batching)
#[cfg(windows)]
pub fn send_string(s: &str) {
    if s.is_empty() {
        return;
    }

    debug!("Sending string: '{}'", s);

    let char_count = s.chars().count();
    let mut inputs = Vec::with_capacity(char_count * 2);

    for ch in s.chars() {
        let code = ch as u32;

        // Handle characters that need surrogate pairs
        if code > 0xFFFF {
            let code = code - 0x10000;
            let high = ((code >> 10) + 0xD800) as u16;
            let low = ((code & 0x3FF) + 0xDC00) as u16;
            add_unicode_inputs(&mut inputs, high);
            add_unicode_inputs(&mut inputs, low);
        } else {
            add_unicode_inputs(&mut inputs, code as u16);
        }
    }

    if !inputs.is_empty() {
        // SAFETY:
        // 1. inputs is a valid Vec<INPUT> with unicode key events
        // 2. as_mut_ptr() returns valid pointer, validated non-empty above
        // 3. expected count matches Vec length
        // 4. size_of::<INPUT>() is correct for SendInput
        // 5. SendInput is properly declared in windows_sys
        // 6. INPUT structs contain valid Unicode scan codes (wScan field)
        // 7. KEYEVENTF_UNICODE flag properly set for unicode input
        unsafe {
            let expected = inputs.len() as u32;
            let sent = SendInput(
                expected,
                inputs.as_mut_ptr(),
                std::mem::size_of::<INPUT>() as i32,
            );

            if sent != expected {
                tracing::error!("SendInput failed: expected {}, sent {}", expected, sent);
            }
        }
    }
}

/// Send backspaces and string in one batch
#[cfg(windows)]
pub fn send_replacement(backspace_count: usize, text: &str) {
    if backspace_count == 0 && text.is_empty() {
        return;
    }

    debug!("Sending replacement: {} backspaces + '{}'", backspace_count, text);
    
    // Calculate total capacity needed
    let char_count = text.chars().count();
    // 2 inputs per backspace (down/up), 2 inputs per char (down/up) + extras for surrogates
    let capacity = (backspace_count * 2) + (char_count * 2); 
    
    let mut inputs = Vec::with_capacity(capacity);

    // 1. Add Backspaces
    for _ in 0..backspace_count {
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_BACK as u16,
                    wScan: 0,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: BUTTRE_INJECTED,
                },
            },
        });

        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_BACK as u16,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: BUTTRE_INJECTED,
                },
            },
        });
    }

    // 2. Add Text
    for ch in text.chars() {
        let code = ch as u32;
        if code > 0xFFFF {
            let code = code - 0x10000;
            let high = ((code >> 10) + 0xD800) as u16;
            let low = ((code & 0x3FF) + 0xDC00) as u16;
            add_unicode_inputs(&mut inputs, high);
            add_unicode_inputs(&mut inputs, low);
        } else {
            add_unicode_inputs(&mut inputs, code as u16);
        }
    }

    // 3. Send all at once
    if !inputs.is_empty() {
        // SAFETY:
        // 1. inputs contains both backspace and unicode key events in single batch
        // 2. as_mut_ptr() returns valid pointer, validated non-empty above
        // 3. expected count matches Vec length
        // 4. size_of::<INPUT>() is correct for SendInput
        // 5. SendInput is properly declared in windows_sys
        // 6. Batching improves performance and timing consistency
        // 7. All INPUT structs properly initialized with correct flags
        unsafe {
            let expected = inputs.len() as u32;
            let sent = SendInput(
                expected,
                inputs.as_mut_ptr(),
                std::mem::size_of::<INPUT>() as i32,
            );

            if sent != expected {
                tracing::error!("SendInput batch failed: expected {}, sent {}", expected, sent);
            }
        }
    }
}

#[cfg(not(windows))]
pub fn send_string(_s: &str) {}

#[cfg(not(windows))]
pub fn send_replacement(_backspace_count: usize, _text: &str) {}

/// Add Unicode key down/up to inputs vector
#[cfg(windows)]
fn add_unicode_inputs(inputs: &mut Vec<INPUT>, scan: u16) {
    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: KEYBDINPUT {
                wVk: 0,
                wScan: scan,
                dwFlags: KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: BUTTRE_INJECTED,
            },
        },
    });

    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: KEYBDINPUT {
                wVk: 0,
                wScan: scan,
                dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: BUTTRE_INJECTED,
            },
        },
    });
}

/// Send a single Unicode character
#[cfg(windows)]
pub fn send_unicode_char(ch: char) {
    send_string(&ch.to_string());
}

#[cfg(not(windows))]
pub fn send_unicode_char(_ch: char) {}
