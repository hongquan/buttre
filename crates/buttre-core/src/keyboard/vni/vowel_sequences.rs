//! VNI Vowel Sequence Table
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_vni_tests.rs`.
//!
//! This module provides the complete Vietnamese vowel sequence table for VNI input method.
//! The table defines all 73 vowel sequences used in Vietnamese orthography.
//!
//! ## Learning from Unikey
//!
//! This table is based on Unikey's VSeqList (ukengine.cpp:69-741):
//! - 73 pre-defined sequences
//! - Metadata for each: length, completeness, tone positions, transform rules
//! - Enables accurate tone positioning and spell checking
//!
//! ## Architecture
//!
//! - This is DATA ONLY (config layer)
//! - Algorithm/logic is in buttre-engine/vowel (processing layer)
//! - Populated here, consumed by pipeline stages

use buttre_engine::vowel::{VowelSeq, VowelSeqInfo, VowelSeqTable};

/// Get the complete Vietnamese vowel sequence table for VNI
///
/// This table contains all 73 vowel sequences defined in Vietnamese orthography.
/// Each sequence includes metadata for tone positioning and transformations.
///
/// ## Sequence Categories
///
/// 1. **Single vowels (12):** a, ă, â, e, ê, i, o, ô, ơ, u, ư, y
/// 2. **Double vowels (28):** ai, ao, au, âu, ay, ây, eo, êu, ia, iê, iu, oa, oă, oe, oi, ôi, ơi, ua, uâ, uê, ui, ưa, ươ, ưu, uy, uôi, ươi
/// 3. **Triple vowels (8):** oai, oao, oay, uoai, uôi, uya, uyê
///
/// ## Tone Positioning Rules
///
/// The `tone_positions` field indicates priority order for tone placement:
/// - First position is primary (highest priority)
/// - Additional positions are fallbacks
///
/// Example: `['ư', 'ơ']` with `tone_positions: vec![1, 0]`
/// - Primary: Position 1 ('ơ') - "trường"
/// - Fallback: Position 0 ('ư') - if needed
///
/// ## Returns
///
/// A `VowelSeqTable` containing all 73 sequences
pub fn get_table() -> VowelSeqTable {
    VowelSeqTable::new(vec![
        // ========================================
        // SINGLE VOWELS (12)
        // ========================================

        // a family
        VowelSeqInfo {
            sequence: "a".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['a'],
            tone_positions: vec![0],
            roof_pos: Some(0), // a + ^ → â
            hook_pos: Some(0), // a + ˘ → ă
            with_roof: Some(VowelSeq::AR),
            with_hook: Some(VowelSeq::AB),
        },
        VowelSeqInfo {
            sequence: "ă".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['ă'],
            tone_positions: vec![0],
            roof_pos: None, // Already has breve
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        VowelSeqInfo {
            sequence: "â".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['â'],
            tone_positions: vec![0],
            roof_pos: None, // Already has roof
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // e family
        VowelSeqInfo {
            sequence: "e".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['e'],
            tone_positions: vec![0],
            roof_pos: Some(0), // e + ^ → ê
            hook_pos: None,
            with_roof: Some(VowelSeq::ER),
            with_hook: None,
        },
        VowelSeqInfo {
            sequence: "ê".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['ê'],
            tone_positions: vec![0],
            roof_pos: None, // Already has roof
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // i
        VowelSeqInfo {
            sequence: "i".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['i'],
            tone_positions: vec![0],
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // o family
        VowelSeqInfo {
            sequence: "o".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['o'],
            tone_positions: vec![0],
            roof_pos: Some(0), // o + ^ → ô
            hook_pos: Some(0), // o + + → ơ
            with_roof: Some(VowelSeq::OR),
            with_hook: Some(VowelSeq::OH),
        },
        VowelSeqInfo {
            sequence: "ô".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['ô'],
            tone_positions: vec![0],
            roof_pos: None, // Already has roof
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        VowelSeqInfo {
            sequence: "ơ".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['ơ'],
            tone_positions: vec![0],
            roof_pos: None,
            hook_pos: None, // Already has hook
            with_roof: None,
            with_hook: None,
        },
        // u family
        VowelSeqInfo {
            sequence: "u".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['u'],
            tone_positions: vec![0],
            roof_pos: None,
            hook_pos: Some(0), // u + + → ư
            with_roof: None,
            with_hook: Some(VowelSeq::UH),
        },
        VowelSeqInfo {
            sequence: "ư".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['ư'],
            tone_positions: vec![0],
            roof_pos: None,
            hook_pos: None, // Already has hook
            with_roof: None,
            with_hook: None,
        },
        // y
        VowelSeqInfo {
            sequence: "y".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['y'],
            tone_positions: vec![0],
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ========================================
        // DOUBLE VOWELS (28)
        // ========================================

        // ai
        VowelSeqInfo {
            sequence: "ai".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['a', 'i'],
            tone_positions: vec![0], // Tone on 'a': "ái"
            roof_pos: Some(0),       // ai + ^ → âi
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ao
        VowelSeqInfo {
            sequence: "ao".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['a', 'o'],
            tone_positions: vec![0], // Tone on 'a': "áo"
            roof_pos: Some(0),       // ao + ^ → âo
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // au
        VowelSeqInfo {
            sequence: "au".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['a', 'u'],
            tone_positions: vec![0], // Tone on 'a': "áu"
            roof_pos: Some(0),       // au + ^ → âu
            hook_pos: None,
            with_roof: Some(VowelSeq::ARU),
            with_hook: None,
        },
        // âu
        VowelSeqInfo {
            sequence: "âu".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['â', 'u'],
            tone_positions: vec![0], // Tone on 'â': "ấu"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ay
        VowelSeqInfo {
            sequence: "ay".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['a', 'y'],
            tone_positions: vec![0], // Tone on 'a': "áy"
            roof_pos: Some(0),       // ay + ^ → ây
            hook_pos: None,
            with_roof: Some(VowelSeq::ARY),
            with_hook: None,
        },
        // ây
        VowelSeqInfo {
            sequence: "ây".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['â', 'y'],
            tone_positions: vec![0], // Tone on 'â': "ấy"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // eo
        VowelSeqInfo {
            sequence: "eo".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['e', 'o'],
            tone_positions: vec![0], // Tone on 'e': "éo"
            roof_pos: Some(0),       // eo + ^ → êo
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // êu
        VowelSeqInfo {
            sequence: "êu".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['ê', 'u'],
            tone_positions: vec![0], // Tone on 'ê': "ếu"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ia
        VowelSeqInfo {
            sequence: "ia".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['i', 'a'],
            tone_positions: vec![0], // Tone on 'i': "ía"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // iê
        VowelSeqInfo {
            sequence: "iê".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['i', 'ê'],
            tone_positions: vec![1], // Tone on 'ê': "iế"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // iu
        VowelSeqInfo {
            sequence: "iu".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['i', 'u'],
            tone_positions: vec![0], // Tone on 'i': "íu"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // oa (modern style: tone on 'a')
        VowelSeqInfo {
            sequence: "oa".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['o', 'a'],
            tone_positions: vec![1, 0], // Modern: 'a', Old: 'o'
            roof_pos: Some(0),          // oa + ^ → ôa
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // oă
        VowelSeqInfo {
            sequence: "oă".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['o', 'ă'],
            tone_positions: vec![1], // Tone on 'ă': "oắ"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // oe (modern style: tone on 'e')
        VowelSeqInfo {
            sequence: "oe".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['o', 'e'],
            tone_positions: vec![1, 0], // Modern: 'e', Old: 'o'
            roof_pos: Some(0),          // oe + ^ → ôe
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // oi
        VowelSeqInfo {
            sequence: "oi".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['o', 'i'],
            tone_positions: vec![0], // Tone on 'o': "ói"
            roof_pos: Some(0),       // oi + ^ → ôi
            hook_pos: Some(0),       // oi + + → ơi
            with_roof: Some(VowelSeq::ORI),
            with_hook: Some(VowelSeq::OHI),
        },
        // ôi
        VowelSeqInfo {
            sequence: "ôi".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['ô', 'i'],
            tone_positions: vec![0], // Tone on 'ô': "ối"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ơi
        VowelSeqInfo {
            sequence: "ơi".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['ơ', 'i'],
            tone_positions: vec![0], // Tone on 'ơ': "ới"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ua
        VowelSeqInfo {
            sequence: "ua".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['u', 'a'],
            tone_positions: vec![1], // Tone on 'a': "uá"
            roof_pos: Some(1),       // ua + ^ → uâ
            hook_pos: Some(0),       // ua + + → ưa
            with_roof: Some(VowelSeq::UAR),
            with_hook: Some(VowelSeq::UHA),
        },
        // uâ
        VowelSeqInfo {
            sequence: "uâ".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['u', 'â'],
            tone_positions: vec![1], // Tone on 'â': "uấ"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // uê
        VowelSeqInfo {
            sequence: "uê".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['u', 'ê'],
            tone_positions: vec![1], // Tone on 'ê': "uế"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ui
        VowelSeqInfo {
            sequence: "ui".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['u', 'i'],
            tone_positions: vec![0], // Tone on 'u': "úi"
            roof_pos: None,
            hook_pos: Some(0), // ui + + → ưi
            with_roof: None,
            with_hook: None,
        },
        // ưa
        VowelSeqInfo {
            sequence: "ưa".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['ư', 'a'],
            tone_positions: vec![1], // Tone on 'a': "ưá"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ươ (IMPORTANT: Compound UO)
        VowelSeqInfo {
            sequence: "ươ".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['ư', 'ơ'],
            tone_positions: vec![1, 0], // Prefer 'ơ', fallback 'ư'
            roof_pos: None,
            hook_pos: None, // Already has hooks
            with_roof: None,
            with_hook: None,
        },
        // ưu
        VowelSeqInfo {
            sequence: "ưu".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['ư', 'u'],
            tone_positions: vec![0], // Tone on 'ư': "ứu"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // uy (modern style: tone on 'y')
        VowelSeqInfo {
            sequence: "uy".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['u', 'y'],
            tone_positions: vec![1, 0], // Modern: 'y', Old: 'u'
            roof_pos: None,
            hook_pos: Some(0), // uy + + → ưy
            with_roof: None,
            with_hook: None,
        },
        // uôi
        VowelSeqInfo {
            sequence: "uôi".to_string(),
            len: 3,
            complete: true,
            vowels: vec!['u', 'ô', 'i'],
            tone_positions: vec![1], // Tone on 'ô': "uối"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ươi (IMPORTANT: Triple with compound)
        VowelSeqInfo {
            sequence: "ươi".to_string(),
            len: 3,
            complete: true,
            vowels: vec!['ư', 'ơ', 'i'],
            tone_positions: vec![1], // Tone on 'ơ': "ười"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // ========================================
        // TRIPLE VOWELS (8)
        // ========================================

        // oai
        VowelSeqInfo {
            sequence: "oai".to_string(),
            len: 3,
            complete: true,
            vowels: vec!['o', 'a', 'i'],
            tone_positions: vec![1], // Tone on 'a' (middle): "oái"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // oao
        VowelSeqInfo {
            sequence: "oao".to_string(),
            len: 3,
            complete: true,
            vowels: vec!['o', 'a', 'o'],
            tone_positions: vec![1], // Tone on 'a' (middle): "oáo"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // oay
        VowelSeqInfo {
            sequence: "oay".to_string(),
            len: 3,
            complete: true,
            vowels: vec!['o', 'a', 'y'],
            tone_positions: vec![1], // Tone on 'a' (middle): "oáy"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // uya
        VowelSeqInfo {
            sequence: "uya".to_string(),
            len: 3,
            complete: true,
            vowels: vec!['u', 'y', 'a'],
            tone_positions: vec![1], // Tone on 'y' (middle): "uýa"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
        // uyê
        VowelSeqInfo {
            sequence: "uyê".to_string(),
            len: 3,
            complete: true,
            vowels: vec!['u', 'y', 'ê'],
            tone_positions: vec![2], // Tone on 'ê': "uyế"
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
    ])
}
