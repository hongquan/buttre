//! Chromium omnibox replace-fix detection.
//!
//! ## The bug (empirically characterized, 2026-07)
//!
//! In a Chromium address bar (omnibox) with inline autocomplete active —
//! typed `d`, ghost suggestion `uckduckgo.com` shown selected — a synthetic
//! `VK_BACK` does NOT delete the typed character: Chromium consumes it to
//! dismiss the autocomplete selection. Our usual transform injection
//! (`N × VK_BACK` + replacement text, see `input::send_replacement`) then
//! under-deletes by one, committing `dđ` instead of `đ`. This is by design
//! on Chromium's side: omnibox/IME coordination only engages for real TSF
//! composition, never for hook-injected keystrokes (Chromium bugs 383093,
//! 514928 — open for a decade). A timing delay does NOT help (verified);
//! the interaction is semantic, not a race.
//!
//! ## The fix (OpenKey's mechanism, verified here per-case)
//!
//! Pre-select the last real character with `Shift+Left`, skip one backspace,
//! and let the replacement text type over the selection (`input::
//! send_replacement`'s selection variant). Verified against a live Chrome
//! omnibox in all three states: autocomplete active (`đ` ✓), no autocomplete
//! (`z` ✓), caret mid-text (`zy` ✓ — where a Delete-based variant eats the
//! following character).
//!
//! ## Scoping — why TWO gates
//!
//! `Shift+Left` select-and-overwrite is only needed in the omnibox, and in
//! canvas-grid apps (Google Sheets ready-mode) arrow chords get intercepted
//! for navigation — OpenKey's own maintainer documents Sheets breakage from
//! applying it browser-wide (tuyenvm/OpenKey#37). We therefore gate on:
//!
//! 1. Foreground exe ∈ Chromium allowlist — cheap kernel query, cached per
//!    PID, keeps every non-browser app off the UIA path entirely.
//! 2. UIA focused element class == `OmniboxViewViews` — the Chromium views
//!    class of the omnibox text field, locale-independent, exposed without
//!    enabling renderer accessibility. Focus in page content reports the
//!    web element's own class instead (verified) — so Sheets/Docs/any web
//!    field can never match, and window-class checks can't do this
//!    (`Chrome_WidgetWin_1` covers omnibox AND page focus — verified).
//!
//! Any failure (UIA unavailable, COM error, process query denied) returns
//! `false` → callers fall back to the plain backspace path, i.e. today's
//! behavior. Never worse, only better.

use std::cell::RefCell;
use std::collections::HashMap;

use windows::core::Interface;
use windows::Win32::Foundation::{CloseHandle, HWND};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
    PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Accessibility::{CUIAutomation, IUIAutomation, IUIAutomation2};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

/// Chromium-based browsers whose omnibox exhibits the autocomplete/backspace
/// interaction. `browser.exe` is Cốc Cốc (the major Vietnamese Chromium
/// browser) and Yandex — both Chromium, both safe to include since gate 2
/// still requires the omnibox class.
const CHROMIUM_EXES: &[&str] = &[
    "chrome.exe",
    "msedge.exe",
    "brave.exe",
    "vivaldi.exe",
    "opera.exe",
    "opera_gx.exe",
    "browser.exe",
    "arc.exe",
];

/// The Chromium views class name of the omnibox text field. Stable across
/// Chromium derivatives (it comes from upstream `OmniboxViewViews`).
const OMNIBOX_CLASS: &str = "OmniboxViewViews";

/// Cap for the PID→exe cache; PIDs recycle, so keep it small and just clear
/// wholesale when it fills. A stale hit is harmless: gate 2 (omnibox class)
/// still decides correctness — worst case we pay one UIA query we didn't
/// need, or skip the fix until the cache clears.
const EXE_CACHE_CAP: usize = 64;

thread_local! {
    /// The hook thread is the only caller; keep everything thread-local so
    /// there is no lock on the keystroke path.
    static EXE_CACHE: RefCell<HashMap<u32, bool>> = RefCell::new(HashMap::new());
    static UIA: RefCell<Option<IUIAutomation>> = const { RefCell::new(None) };
}

/// `true` when the focused control is a Chromium omnibox, i.e. the one place
/// `send_replacement` must use select-and-overwrite instead of raw
/// backspaces. Conservative: any error on any step → `false`.
pub fn should_apply_omnibox_fix() -> bool {
    // SAFETY: GetForegroundWindow has no preconditions; a null HWND is
    // handled by the pid == 0 check below (GetWindowThreadProcessId on a
    // null/invalid HWND simply writes no PID).
    let hwnd = unsafe { GetForegroundWindow() };
    let mut pid = 0u32;
    // SAFETY: hwnd is whatever the OS returned above; pid is a valid
    // out-pointer for the lifetime of the call.
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == 0 {
        return false;
    }

    if !is_chromium_pid(pid) {
        return false;
    }

    focused_element_is_omnibox()
}

/// Gate 1: is the foreground process a known Chromium browser? Cached per
/// PID — one kernel query per new foreground process, not per keystroke.
fn is_chromium_pid(pid: u32) -> bool {
    EXE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(&hit) = cache.get(&pid) {
            return hit;
        }
        let verdict = query_exe_basename(pid)
            .map(|name| is_chromium_exe(&name))
            .unwrap_or(false);
        if cache.len() >= EXE_CACHE_CAP {
            cache.clear();
        }
        cache.insert(pid, verdict);
        verdict
    })
}

/// Pure allowlist check on a lowercase exe basename (unit-tested).
fn is_chromium_exe(basename: &str) -> bool {
    CHROMIUM_EXES.contains(&basename)
}

/// Full image path of `pid`, reduced to its lowercase basename.
fn query_exe_basename(pid: u32) -> Option<String> {
    // SAFETY: OpenProcess with QUERY_LIMITED_INFORMATION needs no special
    // privilege for same-session processes; failure returns Err (handled).
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }.ok()?;

    let mut buf = [0u16; 512];
    let mut len = buf.len() as u32;
    // SAFETY: handle is a live process handle owned by this scope; buf/len
    // form a valid sized buffer; QueryFullProcessImageNameW writes at most
    // `len` u16s and updates len to the actual length.
    let ok = unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            windows::core::PWSTR(buf.as_mut_ptr()),
            &mut len,
        )
    };
    // SAFETY: handle came from a successful OpenProcess above and has not
    // been closed elsewhere.
    unsafe {
        let _ = CloseHandle(handle);
    }
    ok.ok()?;

    let path = String::from_utf16_lossy(&buf[..len as usize]);
    let basename = path.rsplit(['\\', '/']).next().unwrap_or(&path);
    Some(basename.to_ascii_lowercase())
}

/// Gate 2: UIA focused-element class check. Cross-process COM call — only
/// reached when the foreground app is an allowlisted browser AND a transform
/// with backspaces is about to fire (a few times per second at most while
/// typing Vietnamese in a browser).
fn focused_element_is_omnibox() -> bool {
    UIA.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = init_uia();
        }
        let Some(uia) = slot.as_ref() else {
            return false;
        };
        // SAFETY: uia is a live COM interface created on this thread;
        // GetFocusedElement/CurrentClassName are read-only queries whose
        // failures we map to false (fall back to the plain replace path).
        unsafe {
            let Ok(element) = uia.GetFocusedElement() else {
                return false;
            };
            match element.CurrentClassName() {
                Ok(class_name) => class_name.to_string() == OMNIBOX_CLASS,
                Err(_) => false,
            }
        }
    })
}

/// One-time-per-thread UIA setup. The low-level hook must answer within the
/// OS hook timeout, so when the IUIAutomation2 interface is available we cap
/// the cross-process connection wait well below that.
fn init_uia() -> Option<IUIAutomation> {
    // SAFETY: CoInitializeEx on the hook thread; S_FALSE (already
    // initialized) and RPC_E_CHANGED_MODE (already MTA) both leave COM
    // usable for the proxy-based UIA client, so the result is ignored.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
    }
    // SAFETY: standard COM activation of the documented CUIAutomation
    // coclass; a failure yields None and the fix stays disabled.
    let uia: IUIAutomation =
        unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER) }.ok()?;

    if let Ok(uia2) = uia.cast::<IUIAutomation2>() {
        // SAFETY: setting client-side timeout properties on a live
        // interface; failure only means we keep UIA's default timeout.
        unsafe {
            let _ = uia2.SetConnectionTimeout(100);
            let _ = uia2.SetTransactionTimeout(100);
        }
    }
    Some(uia)
}

/// Best-effort probe used by diagnostics/logging call sites if ever needed.
#[allow(dead_code)]
pub fn foreground_hwnd() -> HWND {
    // SAFETY: no preconditions.
    unsafe { GetForegroundWindow() }
}

#[cfg(test)]
mod tests {
    use super::is_chromium_exe;

    #[test]
    fn allowlist_matches_known_chromium_browsers() {
        for exe in ["chrome.exe", "msedge.exe", "brave.exe", "browser.exe"] {
            assert!(is_chromium_exe(exe), "{exe} should be allowlisted");
        }
    }

    #[test]
    fn allowlist_rejects_non_browsers_and_near_misses() {
        for exe in [
            "firefox.exe", // Gecko: focus classes differ, mechanism unverified there
            "notepad.exe",
            "chrome",     // no extension — basename must be exact
            "chrome.exe.bak",
            "CHROME.EXE", // caller lowercases; raw uppercase must not match
        ] {
            assert!(!is_chromium_exe(exe), "{exe} must NOT be allowlisted");
        }
    }
}
