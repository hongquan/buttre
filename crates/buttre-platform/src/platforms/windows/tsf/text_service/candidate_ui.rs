//! Candidate UI for Windows TSF
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-platform/tests/platform_windows_tsf_tests.rs`.
//!
//! Implementation of ITfCandidateListUIElement for displaying
//! Nôm character candidates.

use std::cell::{Cell, RefCell};
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::TextServices::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// Candidate UI Element for Nôm characters
#[implement(ITfUIElement, ITfCandidateListUIElement)]
pub struct NomCandidateUI {
    /// List of candidates to display
    candidates: Vec<CandidateItem>,

    /// Currently selected index - uses Cell for interior mutability
    selected_index: Cell<usize>,

    /// Page size (how many candidates to show at once)
    page_size: usize,

    /// Current page index - uses Cell for interior mutability
    current_page: Cell<usize>,

    /// Window handle (for Win32 window) - uses RefCell for interior mutability
    hwnd: RefCell<Option<HWND>>,

    /// Visibility state - uses Cell for interior mutability
    is_shown: Cell<bool>,

    /// Callback for when a candidate is selected (character, reading)
    #[allow(clippy::type_complexity)]
    on_select: RefCell<Option<Box<dyn Fn(char, &str)>>>,
}

/// Single candidate item
#[derive(Clone)]
pub struct CandidateItem {
    /// The Nôm character
    pub character: char,

    /// Phonetic reading (for display)
    pub reading: String,

    /// Meaning/definition (optional)
    pub meaning: Option<String>,

    /// Frequency/rank (for sorting)
    pub frequency: u32,
}

impl Drop for NomCandidateUI {
    fn drop(&mut self) {
        // Destroy the Win32 window before the NomCandidateUI memory is freed.
        // This prevents window_proc from dereferencing GWLP_USERDATA after the backing struct is gone.
        let _ = self.destroy_window();
    }
}

impl NomCandidateUI {
    /// Create new candidate UI
    pub fn new(candidates: Vec<CandidateItem>) -> Self {
        Self {
            candidates,
            selected_index: Cell::new(0),
            page_size: 9, // Show 9 candidates (1-9 keys)
            current_page: Cell::new(0),
            hwnd: RefCell::new(None),
            is_shown: Cell::new(false),
            on_select: RefCell::new(None),
        }
    }

    /// Get current page of candidates
    pub fn current_page_candidates(&self) -> &[CandidateItem] {
        let start = self.current_page.get() * self.page_size;
        let end = (start + self.page_size).min(self.candidates.len());
        &self.candidates[start..end]
    }

    /// Get total number of pages
    pub fn page_count(&self) -> usize {
        self.candidates.len().div_ceil(self.page_size)
    }

    /// Move to next page
    pub fn next_page(&self) -> bool {
        let current = self.current_page.get();
        if current + 1 < self.page_count() {
            self.current_page.set(current + 1);
            self.selected_index
                .set(self.current_page.get() * self.page_size);
            true
        } else {
            false
        }
    }

    /// Move to previous page
    pub fn prev_page(&self) -> bool {
        let current = self.current_page.get();
        if current > 0 {
            self.current_page.set(current - 1);
            self.selected_index
                .set(self.current_page.get() * self.page_size);
            true
        } else {
            false
        }
    }

    /// Select candidate by index (0-8 for current page)
    pub fn select(&self, page_index: usize) -> Option<&CandidateItem> {
        let global_index = self.current_page.get() * self.page_size + page_index;
        if global_index < self.candidates.len() {
            self.selected_index.set(global_index);
            Some(&self.candidates[global_index])
        } else {
            None
        }
    }

    /// Get selected candidate
    pub fn selected(&self) -> Option<&CandidateItem> {
        self.candidates.get(self.selected_index.get())
    }

    /// Set callback for when a candidate is selected
    pub fn set_on_select<F>(&self, callback: F)
    where
        F: Fn(char, &str) + 'static,
    {
        *self.on_select.borrow_mut() = Some(Box::new(callback));
    }

    /// Trigger selection callback
    fn trigger_selection(&self, candidate: &CandidateItem) {
        if let Some(ref callback) = *self.on_select.borrow() {
            callback(candidate.character, &candidate.reading);
        }
    }

    /// Create and show candidate window
    pub fn create_window(&self, x: i32, y: i32) -> Result<()> {
        // SAFETY:
        // 1. RegisterClassW is called once via std::sync::Once - thread-safe initialization
        // 2. WNDCLASSW is properly initialized with valid function pointer and strings
        // 3. CreateWindowExW creates a native Windows window with valid parameters
        // 4. w!() macro creates valid null-terminated wide strings
        // 5. SetWindowLongPtrW stores pointer to self - valid as long as window exists
        // 6. ShowWindow and UpdateWindow are standard Win32 APIs - safe to call
        // 7. Window is destroyed before self is dropped (in destroy_window)
        unsafe {
            // Register window class (one-time)
            use std::sync::Once;
            static REGISTER: Once = Once::new();

            REGISTER.call_once(|| {
                let wc = WNDCLASSW {
                    lpfnWndProc: Some(window_proc),
                    lpszClassName: w!("buttreNomCandidate"),
                    hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
                    hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize as *mut _),
                    ..Default::default()
                };
                let _ = RegisterClassW(&wc);
            });

            // Create window
            let hwnd = CreateWindowExW(
                WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW,
                w!("buttreNomCandidate"),
                w!(""),
                WS_POPUP | WS_BORDER,
                x,
                y,
                400,
                250, // Position and size
                None,
                None,
                None,
                None,
            )?;

            // Store window handle
            *self.hwnd.borrow_mut() = Some(hwnd);

            // Store pointer to self in window user data for window procedure access
            let ui_ptr = self as *const NomCandidateUI as isize;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, ui_ptr);

            // Show window
            let _ = ShowWindow(hwnd, SW_SHOW);
            let _ = UpdateWindow(hwnd);
            self.is_shown.set(true);

            Ok(())
        }
    }

    /// Hide and destroy window
    pub fn destroy_window(&self) -> Result<()> {
        if let Some(hwnd) = *self.hwnd.borrow() {
            // SAFETY:
            // 1. hwnd is a valid HWND from CreateWindowExW
            // 2. Clearing GWLP_USERDATA BEFORE DestroyWindow ensures that any
            //    WM_PAINT / WM_ERASEBKGND messages pumped synchronously during
            //    DestroyWindow see a null pointer and skip dereferencing self.
            //    (WM_NCDESTROY also clears it, but only arrives late in destruction.)
            // 3. DestroyWindow is a standard Win32 API
            // 4. After DestroyWindow we set hwnd to None (no double-destroy)
            unsafe {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                DestroyWindow(hwnd)?;
            }
            *self.hwnd.borrow_mut() = None;
            self.is_shown.set(false);
        }
        Ok(())
    }
}

#[allow(non_snake_case)]
impl ITfUIElement_Impl for NomCandidateUI_Impl {
    fn GetDescription(&self) -> Result<BSTR> {
        Ok(BSTR::from("buttre Nôm Candidate Window"))
    }

    fn GetGUID(&self) -> Result<windows::core::GUID> {
        // Return a unique GUID for this UI element
        // Generated GUID: {A5B3C4D5-E6F7-8901-2345-6789ABCDEF01}
        Ok(windows::core::GUID::from_values(
            0xA5B3C4D5,
            0xE6F7,
            0x8901,
            [0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01],
        ))
    }

    fn Show(&self, bshow: BOOL) -> Result<()> {
        // Update visibility state using Cell
        self.is_shown.set(bshow.as_bool());

        // If we have a window handle, show/hide it
        if let Some(hwnd) = *self.hwnd.borrow() {
            // SAFETY:
            // 1. hwnd is a valid HWND from CreateWindowExW
            // 2. ShowWindow is a standard Win32 API - safe to call
            // 3. SW_SHOW and SW_HIDE are valid window show commands
            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOW};
                let _ = ShowWindow(hwnd, if bshow.as_bool() { SW_SHOW } else { SW_HIDE });
            }
        }

        Ok(())
    }

    fn IsShown(&self) -> Result<BOOL> {
        // Return visibility state using Cell
        Ok(BOOL::from(self.is_shown.get()))
    }
}

#[allow(non_snake_case)]
impl ITfCandidateListUIElement_Impl for NomCandidateUI_Impl {
    fn GetUpdatedFlags(&self) -> Result<u32> {
        // Return flags indicating what has changed
        Ok(TF_CLUIE_DOCUMENTMGR | TF_CLUIE_COUNT | TF_CLUIE_SELECTION)
    }

    fn GetDocumentMgr(&self) -> Result<ITfDocumentMgr> {
        // TODO: Return the document manager
        Err(E_NOTIMPL.into())
    }

    fn GetCount(&self) -> Result<u32> {
        Ok(self.candidates.len() as u32)
    }

    fn GetSelection(&self) -> Result<u32> {
        Ok(self.selected_index.get() as u32)
    }

    fn GetString(&self, index: u32) -> Result<BSTR> {
        let candidate = self.candidates.get(index as usize).ok_or(E_INVALIDARG)?;

        // Format: "1. 𡦂 (người) - person"
        let display = if let Some(ref meaning) = candidate.meaning {
            format!(
                "{}. {} ({}) - {}",
                (index % self.page_size as u32) + 1,
                candidate.character,
                candidate.reading,
                meaning
            )
        } else {
            format!(
                "{}. {} ({})",
                (index % self.page_size as u32) + 1,
                candidate.character,
                candidate.reading
            )
        };

        Ok(BSTR::from(display))
    }

    // Signature is fixed by the windows-rs-generated
    // `ITfCandidateListUIElement_Impl` trait (COM vtable contract) — cannot
    // be `unsafe fn`. Raw pointer writes are scoped to an inner `unsafe`
    // block below.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn GetPageIndex(&self, _pindex: *mut u32, _usize: u32, _pupagecnt: *mut u32) -> Result<()> {
        // SAFETY:
        // 1. _pindex and _pupagecnt pointers are provided by COM caller
        // 2. We check for null before dereferencing
        // 3. Writing u32 values to these pointers is safe
        // 4. current_page and page_count are valid values within bounds
        unsafe {
            if !_pindex.is_null() {
                *_pindex = self.current_page.get() as u32;
            }
            if !_pupagecnt.is_null() {
                *_pupagecnt = self.page_count() as u32;
            }
        }
        Ok(())
    }

    fn SetPageIndex(&self, _pindex: *const u32, _upagecnt: u32) -> Result<()> {
        // TODO: Implement page navigation
        Ok(())
    }

    fn GetCurrentPage(&self) -> Result<u32> {
        Ok(self.current_page.get() as u32)
    }
}

/// Window procedure for candidate window
// SAFETY:
// 1. This is a Windows window procedure - must use extern "system" calling convention
// 2. Called by Windows OS with valid hwnd and parameters
// 3. All Win32 GDI/window functions are properly declared in windows crate
// 4. ui_ptr retrieved from GWLP_USERDATA is valid as long as window exists
// 5. Must return LRESULT per Windows window procedure protocol
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // A panic crossing the FFI boundary is undefined behaviour. Catch it and
    // fall through to DefWindowProcW so the host process remains stable.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: same guarantees as window_proc — hwnd/msg/wparam/lparam are valid
        // Win32 parameters provided by the OS.
        unsafe { window_proc_inner(hwnd, msg, wparam, lparam) }
    }));
    match result {
        Ok(v) => v,
        Err(_) => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn window_proc_inner(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            // SAFETY: BeginPaint is safe because hwnd is valid from CreateWindowExW
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

            // Get UI pointer from window user data
            // SAFETY: GetWindowLongPtrW is safe because hwnd is valid
            let ui_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };

            if ui_ptr != 0 {
                // SAFETY: ui_ptr was stored in GWLP_USERDATA by create_window, valid during window lifetime
                let ui = unsafe { &*(ui_ptr as *const NomCandidateUI) };

                // Use system default GUI font
                // SAFETY: GetStockObject is safe with DEFAULT_GUI_FONT constant
                let stock_font = unsafe { GetStockObject(DEFAULT_GUI_FONT) };
                // SAFETY: SelectObject is safe with valid hdc and stock object
                let old_font = unsafe { SelectObject(hdc, stock_font) };

                // Draw background
                let rect = RECT {
                    left: 0,
                    top: 0,
                    right: 400,
                    bottom: 250,
                };
                // SAFETY: FillRect is safe with valid hdc, rect reference, and brush handle
                unsafe { FillRect(hdc, &rect, HBRUSH((COLOR_WINDOW.0 + 1) as isize as *mut _)) };

                // Draw candidates
                let candidates = ui.current_page_candidates();
                let mut y = 10;

                for (i, candidate) in candidates.iter().enumerate() {
                    // Highlight selected
                    let global_index = ui.current_page.get() * ui.page_size + i;
                    if global_index == ui.selected_index.get() {
                        // SAFETY: SetBkColor is safe with valid hdc and color value
                        unsafe { SetBkColor(hdc, COLORREF(0x00FFE4B5)) }; // Light orange
                                                                          // SAFETY: SetTextColor is safe with valid hdc and color value
                        unsafe { SetTextColor(hdc, COLORREF(0x00000000)) }; // Black
                    } else {
                        // SAFETY: SetBkColor is safe with valid hdc and color value
                        unsafe { SetBkColor(hdc, COLORREF(0x00FFFFFF)) }; // White
                                                                          // SAFETY: SetTextColor is safe with valid hdc and color value
                        unsafe { SetTextColor(hdc, COLORREF(0x00000000)) }; // Black
                    }

                    // Format text: "1. 𠊛 (người) - person"
                    let text = if let Some(ref meaning) = candidate.meaning {
                        format!(
                            "{}. {} ({}) - {}",
                            i + 1,
                            candidate.character,
                            candidate.reading,
                            meaning
                        )
                    } else {
                        format!("{}. {} ({})", i + 1, candidate.character, candidate.reading)
                    };

                    // Draw text
                    let text_utf16: Vec<u16> = text.encode_utf16().collect();
                    // SAFETY: TextOutW is safe with valid hdc, coordinates, and UTF-16 text buffer
                    let _ = unsafe { TextOutW(hdc, 10, y, &text_utf16) };

                    y += 25;
                }

                // Draw page info at bottom
                if ui.page_count() > 1 {
                    let page_info =
                        format!("Trang {} / {}", ui.current_page.get() + 1, ui.page_count());
                    let page_utf16: Vec<u16> = page_info.encode_utf16().collect();
                    // SAFETY: SetTextColor is safe with valid hdc and color value
                    unsafe { SetTextColor(hdc, COLORREF(0x00808080)) }; // Gray
                                                                        // SAFETY: TextOutW is safe with valid hdc, coordinates, and UTF-16 text buffer
                    let _ = unsafe { TextOutW(hdc, 10, 220, &page_utf16) };
                }

                // Cleanup
                // SAFETY: SelectObject is safe with valid hdc and previous font object
                unsafe { SelectObject(hdc, old_font) };
                // No DeleteObject needed for stock objects
            }

            // SAFETY: EndPaint is safe and must be called after BeginPaint
            let _ = unsafe { EndPaint(hwnd, &ps) };
            LRESULT(0)
        }
        WM_KEYDOWN => {
            let vk = wparam.0 as u32;
            // SAFETY: GetWindowLongPtrW is safe because hwnd is valid
            let ui_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };

            if ui_ptr != 0 {
                // SAFETY: ui_ptr was stored in GWLP_USERDATA by create_window, valid during window lifetime
                let ui = unsafe { &*(ui_ptr as *const NomCandidateUI) };

                match vk {
                    // Number keys 1-9 for selection
                    0x31..=0x39 => {
                        // VK_1 to VK_9
                        let index = (vk - 0x31) as usize;
                        if let Some(candidate) = ui.select(index) {
                            // Trigger callback to insert character
                            ui.trigger_selection(candidate);

                            // Close the window
                            // SAFETY: DestroyWindow is safe because hwnd is valid
                            let _ = unsafe { DestroyWindow(hwnd) };
                        }
                    }
                    0x22 => {
                        // VK_NEXT (PageDown)
                        if ui.next_page() {
                            // SAFETY: InvalidateRect is safe with valid hwnd
                            let _ = unsafe { InvalidateRect(Some(hwnd), None, true) };
                        }
                    }
                    0x21 => {
                        // VK_PRIOR (PageUp)
                        if ui.prev_page() {
                            // SAFETY: InvalidateRect is safe with valid hwnd
                            let _ = unsafe { InvalidateRect(Some(hwnd), None, true) };
                        }
                    }
                    0x1B => {
                        // VK_ESCAPE
                        // SAFETY: DestroyWindow is safe because hwnd is valid
                        let _ = unsafe { DestroyWindow(hwnd) };
                    }
                    _ => {}
                }
            }

            LRESULT(0)
        }
        WM_DESTROY => LRESULT(0),
        WM_NCDESTROY => {
            // Zero GWLP_USERDATA so any late-arriving messages see a null pointer.
            unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
            LRESULT(0)
        }
        _ => {
            // SAFETY: DefWindowProcW is safe with valid hwnd and message parameters
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
    }
}
