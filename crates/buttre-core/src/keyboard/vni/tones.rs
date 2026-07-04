//! VNI Tone Mappings
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_vni_tests.rs`.
//!
//! This module defines all VNI tone mark mappings.

use buttre_engine::pipeline::config::ToneMark;
use std::collections::HashMap;

/// Get all VNI tone mappings
///
/// ## Returns
/// HashMap mapping input characters to tone marks
///
/// ## Examples
/// - '1' → Acute (sắc)
/// - '2' → Grave (huyền)
/// - '3' → Hook (hỏi)
/// - '4' → Tilde (ngã)
/// - '5' → Dot (nặng)
/// - '0' → None (remove tone)
pub fn get_map() -> HashMap<char, ToneMark> {
    let mut map = HashMap::new();

    map.insert('1', ToneMark::Acute);
    map.insert('2', ToneMark::Grave);
    map.insert('3', ToneMark::Hook);
    map.insert('4', ToneMark::Tilde);
    map.insert('5', ToneMark::Dot);
    map.insert('0', ToneMark::None);

    map
}
