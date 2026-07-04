//! Vowel Sequence Data Structures
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/vowel_sequences_tests.rs`.
//!
//! This module defines the core data structures for Vietnamese vowel sequences.
//! It provides a shared definition that can be used by both:
//! - Config layer (buttre-core): Populate the table with Vietnamese vowel data
//! - Pipeline layer (buttre-engine): Consume the table for processing
//!
//! ## Learning from Unikey
//!
//! This design is inspired by Unikey's VowelSeqInfo table:
//! - 73 pre-defined vowel sequences
//! - Each sequence has metadata (length, tone positions, transform rules)
//! - Enables O(log n) lookup instead of dynamic calculation
//!
//! Reference: `.reference/fcitx5-unikey/unikey/ukengine.cpp:69-741`

use std::fmt;

/// Vietnamese Vowel Sequence
///
/// Represents all possible vowel combinations in Vietnamese orthography.
///
/// ## Examples
///
/// - Single: a, ă, â, e, ê, i, o, ô, ơ, u, ư, y
/// - Double: ai, ao, au, âu, ay, ây, eo, êu, ia, iê, iu, oa, oă, oe, oi, ôi, ơi, ua, uâ, uê, ui, ưa, ươ, ưu, uy, uya, uyê
/// - Triple: oai, oao, oay, uây, uôi, ươi, uya, uyê
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VowelSeq {
    // Single vowels (12)
    /// Single vowel: a
    A,
    /// Single vowel with breve: ă
    AB,
    /// Single vowel with circumflex: â
    AR,
    /// Single vowel: e
    E,
    /// Single vowel with circumflex: ê
    ER,
    /// Single vowel: i
    I,
    /// Single vowel: o
    O,
    /// Single vowel with circumflex: ô
    OR,
    /// Single vowel with horn: ơ
    OH,
    /// Single vowel: u
    U,
    /// Single vowel with horn: ư
    UH,
    /// Single vowel: y
    Y,

    // Double vowels (28)
    /// Double vowel: ai
    AI,
    /// Double vowel: ao
    AO,
    /// Double vowel: au
    AU,
    /// Double vowel: âu
    ARU,
    /// Double vowel: ay
    AY,
    /// Double vowel: ây
    ARY,
    /// Double vowel: eo
    EO,
    /// Double vowel: êu
    ERU,
    /// Double vowel: ia
    IA,
    /// Double vowel: iê
    IER,
    /// Double vowel: iu
    IU,
    /// Double vowel: oa
    OA,
    /// Double vowel: oă
    OAB,
    /// Double vowel: oe
    OE,
    /// Double vowel: oi
    OI,
    /// Double vowel: ôi
    ORI,
    /// Double vowel: ơi
    OHI,
    /// Double vowel: ua
    UA,
    /// Double vowel: uâ
    UAR,
    /// Double vowel: uê
    UER,
    /// Double vowel: ui
    UI,
    /// Double vowel: ưa
    UHA,
    /// Double vowel: ươ
    UHO,
    /// Double vowel: ưu
    UHU,
    /// Double vowel: uy
    UY,
    /// Double vowel: uôi
    UOI,
    /// Double vowel: ươi
    UHOI,

    // Triple vowels (8)
    /// Triple vowel: oai
    OAI,
    /// Triple vowel: oao
    OAO,
    /// Triple vowel: oay
    OAY,
    /// Triple vowel: uoai
    UOAI,
    /// Triple vowel: uôi
    UORI,
    /// Triple vowel: uya
    UAYA,
    /// Triple vowel: uyê
    UYER,

    /// No vowel sequence
    Nil,
}

/// Mark Type
///
/// Represents the type of diacritical mark in Vietnamese.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    /// Roof mark (^): a→â, e→ê, o→ô
    Roof,

    /// Hook mark (+): o→ơ, u→ư
    Hook,

    /// Breve mark (˘): a→ă
    Breve,

    /// Stroke mark (đ): d→đ
    Stroke,

    /// Tone marks (see ToneMark in config.rs)
    Tone(char), // s, f, r, x, j for Telex; 1-5 for VNI
}

/// Vowel Sequence Information
///
/// Contains metadata about a Vietnamese vowel sequence.
///
/// ## Fields
///
/// - `sequence`: The vowel sequence as a string (e.g., "uơng", "oa", "iê")
/// - `len`: Number of vowels in the sequence (1-3)
/// - `complete`: Whether this is a complete/valid sequence for spell check
/// - `vowels`: Individual vowel characters in the sequence
/// - `tone_positions`: Valid positions for tone marks (indices into `vowels`)
/// - `roof_pos`: Position that can receive ^ mark (if any)
/// - `hook_pos`: Position that can receive + mark (if any)
/// - `with_roof`: What sequence this becomes when ^ is added
/// - `with_hook`: What sequence this becomes when + is added
///
/// ## Example
///
/// ```rust,ignore
/// VowelSeqInfo {
///     sequence: "ươ".to_string(),
///     len: 2,
///     complete: true,
///     vowels: vec!['ư', 'ơ'],
///     tone_positions: vec![1, 0],  // Prefer 'ơ', fallback to 'ư'
///     roof_pos: None,  // Already has marks
///     hook_pos: None,
///     with_roof: Some(VowelSeq::UHOR),
///     with_hook: None,
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VowelSeqInfo {
    /// The vowel sequence as a string
    pub sequence: String,

    /// Length of the sequence (1-3)
    pub len: usize,

    /// Whether this is a complete/valid sequence
    /// Used for spell checking (incomplete sequences may need more vowels)
    pub complete: bool,

    /// Individual vowel characters
    pub vowels: Vec<char>,

    /// Valid positions for tone marks (priority order)
    /// Example: [1, 0] means "prefer position 1, fallback to 0"
    pub tone_positions: Vec<usize>,

    /// Position that can receive ^ (roof) mark
    pub roof_pos: Option<usize>,

    /// Position that can receive + (hook/horn) mark
    pub hook_pos: Option<usize>,

    /// Sequence when ^ is added
    pub with_roof: Option<VowelSeq>,

    /// Sequence when + is added
    pub with_hook: Option<VowelSeq>,
}

impl VowelSeqInfo {
    /// Check if a position can receive a tone mark
    ///
    /// ## Arguments
    ///
    /// - `pos`: Position index (0-based)
    ///
    /// ## Returns
    ///
    /// `true` if this position is in `tone_positions`
    pub fn can_receive_tone(&self, pos: usize) -> bool {
        self.tone_positions.contains(&pos)
    }

    /// Get the primary tone position
    ///
    /// Returns the first position in `tone_positions` (highest priority)
    pub fn primary_tone_position(&self) -> Option<usize> {
        self.tone_positions.first().copied()
    }

    /// Check if this sequence can receive a roof mark (^)
    pub fn can_receive_roof(&self) -> bool {
        self.roof_pos.is_some()
    }

    /// Check if this sequence can receive a hook mark (+)
    pub fn can_receive_hook(&self) -> bool {
        self.hook_pos.is_some()
    }
}

impl fmt::Display for VowelSeqInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (len={}, complete={}, tone_pos={:?})",
            self.sequence, self.len, self.complete, self.tone_positions
        )
    }
}

/// Vowel Sequence Table
///
/// A collection of vowel sequences with lookup capabilities.
///
/// ## Usage
///
/// This table is typically populated in the config layer (buttre-core/keyboard)
/// and consumed by the pipeline layer (buttre-engine/pipeline).
///
/// ```rust,ignore
/// // In buttre-core/keyboard/telex/vowel_sequences.rs
/// pub fn get_table() -> VowelSeqTable {
///     VowelSeqTable::new(vec![
///         VowelSeqInfo { sequence: "a".to_string(), ... },
///         VowelSeqInfo { sequence: "ă".to_string(), ... },
///         // ... 73 sequences
///     ])
/// }
///
/// // In buttre-engine/pipeline/config.rs
/// config.tone.vowel_sequences = telex::vowel_sequences::get_table();
/// ```
#[derive(Debug, Clone)]
pub struct VowelSeqTable {
    sequences: Vec<VowelSeqInfo>,
}

impl VowelSeqTable {
    /// Create a new vowel sequence table
    pub fn new(sequences: Vec<VowelSeqInfo>) -> Self {
        Self { sequences }
    }

    /// Create an empty table
    pub fn empty() -> Self {
        Self {
            sequences: Vec::new(),
        }
    }

    /// Find a vowel sequence by its string representation
    ///
    /// ## Algorithm
    ///
    /// Linear search through the table. For performance-critical code,
    /// consider using a HashMap-backed implementation.
    ///
    /// ## Arguments
    ///
    /// - `seq`: The sequence string to find (e.g., "ươ", "oa")
    ///
    /// ## Returns
    ///
    /// Reference to the VowelSeqInfo if found
    pub fn find(&self, seq: &str) -> Option<&VowelSeqInfo> {
        self.sequences.iter().find(|s| s.sequence == seq)
    }

    /// Find a vowel sequence by vowel characters
    ///
    /// ## Arguments
    ///
    /// - `vowels`: Slice of vowel characters (1-3 chars)
    ///
    /// ## Returns
    ///
    /// Reference to the VowelSeqInfo if found
    pub fn find_by_vowels(&self, vowels: &[char]) -> Option<&VowelSeqInfo> {
        self.sequences.iter().find(|s| s.vowels == vowels)
    }

    /// Get all sequences
    pub fn all(&self) -> &[VowelSeqInfo] {
        &self.sequences
    }

    /// Get the number of sequences in the table
    pub fn len(&self) -> usize {
        self.sequences.len()
    }

    /// Check if the table is empty
    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }
}

impl Default for VowelSeqTable {
    fn default() -> Self {
        Self::empty()
    }
}
