//! Unicode Normalization Utilities
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/unicode_normalization_tests.rs`.
//!
//! Handles cross-platform Unicode normalization differences

use unicode_normalization::UnicodeNormalization;

/// Normalize string to NFC (Normalization Form C)
///
/// This is important for cross-platform compatibility:
/// - Windows: UTF-16LE, NFC
/// - Linux: UTF-8, NFC
/// - macOS: UTF-8, NFD (different!)
///
/// Always normalize to NFC for comparison and storage.
pub fn normalize_nfc(s: &str) -> String {
    s.nfc().collect()
}

/// Normalize string to NFD (Normalization Form D)
///
/// Use this when interfacing with macOS filesystem.
pub fn normalize_nfd(s: &str) -> String {
    s.nfd().collect()
}

/// Compare strings with normalization
///
/// Ensures "é" (NFC) equals "é" (NFD)
pub fn str_eq_normalized(a: &str, b: &str) -> bool {
    normalize_nfc(a) == normalize_nfc(b)
}

/// Sanitize filename for cross-platform compatibility
///
/// Removes forbidden characters for Windows, Linux, macOS
pub fn sanitize_filename(name: &str) -> String {
    normalize_nfc(name)
        .chars()
        .map(|c| match c {
            // Windows forbidden
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            // Control characters
            c if c.is_control() => '_',
            // Otherwise keep
            c => c,
        })
        .collect()
}
