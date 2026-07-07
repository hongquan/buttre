//! Key Classification Utilities
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-platform/tests/platform_windows_common_tests.rs`.
//!
//! Functions to classify virtual keys into categories

use super::vk_codes::*;

/// Check if virtual key is a movement key (arrows, home, end, page up/down)
/// These keys should reset the input buffer
#[inline]
pub fn is_movement_key(vk: u16) -> bool {
    matches!(
        vk,
        VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN | VK_HOME | VK_END | VK_PRIOR | VK_NEXT
    )
}

/// Check if virtual key is a modifier key
#[inline]
pub fn is_modifier_key(vk: u16) -> bool {
    matches!(
        vk,
        VK_SHIFT
            | VK_CONTROL
            | VK_MENU
            | VK_LSHIFT
            | VK_RSHIFT
            | VK_LCONTROL
            | VK_RCONTROL
            | VK_LMENU
            | VK_RMENU
            | VK_LWIN
            | VK_RWIN
    )
}

/// Check if virtual key is a special key (not character, but not backspace or movement)
#[inline]
pub fn is_special_key(vk: u16) -> bool {
    matches!(
        vk,
        // Function keys
        VK_F1
            ..=VK_F24 |
        // Insert, Delete
        VK_INSERT | VK_DELETE |
        // System keys (but NOT movement keys)
        VK_ESCAPE | VK_TAB | VK_RETURN |
        VK_PAUSE | VK_SNAPSHOT |
        // Modifier keys
        VK_SHIFT | VK_CONTROL | VK_MENU |
        VK_LSHIFT | VK_RSHIFT |
        VK_LCONTROL | VK_RCONTROL |
        VK_LMENU | VK_RMENU |
        VK_LWIN | VK_RWIN |
        // Lock keys
        VK_CAPITAL | VK_NUMLOCK | VK_SCROLL
    )
}

/// Check if virtual key should cause buffer reset (word termination)
///
/// Based on UniKey behavior, these keys should reset the Vietnamese input buffer:
/// - Movement keys (arrows, Home, End, PgUp/Down) - cursor relocation = word boundary
/// - Enter key (line termination)
/// - Tab key (field navigation)  
/// - Escape key (cancel operation)
/// - Delete key (forward delete)
/// - Insert key (mode toggle)
/// - Function keys (F1-F24) - typically trigger commands
///
/// Note: Mouse clicks are handled separately via mouse hook.
/// Note: Backspace is NOT included (handled separately with undo logic).
/// Note: Space is NOT included (processed as character for soft separator).
#[inline]
pub fn is_buffer_reset_key(vk: u16) -> bool {
    matches!(
        vk,
        // Movement keys (cursor relocation = word boundary)
        VK_LEFT | VK_UP | VK_RIGHT | VK_DOWN |
        VK_HOME | VK_END | VK_PRIOR | VK_NEXT |
        // Line/field terminators
        VK_RETURN | VK_TAB | VK_ESCAPE |
        // Editing keys (non-backspace)
        VK_INSERT | VK_DELETE |
        // Function keys (typically trigger commands)
        VK_F1
            ..=VK_F24 |
        // System keys
        VK_PAUSE | VK_SNAPSHOT
    )
}
