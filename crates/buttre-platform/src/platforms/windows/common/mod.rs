//! buttre Windows Common Utilities
//!
//! Shared code for Windows backends (Hook, TSF)
//! - VK (Virtual Key) constants
//! - Key classification (movement, special, modifier)
//! - Input simulation (SendInput wrappers)
//! - Candidate window (for Nôm input)

#![cfg(windows)]

pub mod input;
pub mod key_utils;
pub mod omnibox_fix;
pub mod vk_codes;
pub mod candidate_window;

pub use input::{send_backspaces, send_string, send_unicode_char, send_replacement};
pub use key_utils::{is_buffer_reset_key, is_modifier_key, is_movement_key, is_special_key};
pub use vk_codes::*;
pub use candidate_window::{show_candidates, hide_candidates, is_showing as is_candidates_showing, select_candidate, get_candidates_text_len, get_input_text_len, get_candidates_count};
