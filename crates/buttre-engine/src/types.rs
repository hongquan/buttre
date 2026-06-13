//! Core type definitions for the buttre-engine
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/types_tests.rs`.
//!
//! This module contains the fundamental types used throughout the engine:
//! - `Action`: Output actions for the platform layer
//! - `WordForm`: Word classification types
//! - `CharInfo`: Character metadata
//! - `Config`: Engine configuration

/// Word form classification (generic, can be used by any language)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordForm {
    /// Not recognized
    NonVn,
    /// Empty
    Empty,
    /// Consonant only
    C,
    /// Vowel only
    V,
    /// Consonant + Vowel
    CV,
    /// Vowel + Consonant
    VC,
    /// Consonant + Vowel + Consonant
    CVC,
}

/// Action to be performed by the IME
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Do nothing
    DoNothing,

    /// Commit text as-is
    Commit(String),

    /// Replace previous text (simulated edit)
    Replace {
        /// Number of characters to delete (backspace)
        backspace_count: usize,
        /// New text to insert
        text: String,
    },

    /// Update TSF composition string (pre-edit)
    UpdateComposition {
        /// Current composition text
        text: String,
        /// Cursor position relative to start of composition
        cursor: usize,
    },
    
    /// Confirm TSF composition (end of word)
    ConfirmComposition(String),
    
    /// Show candidates UI (for Hook mode fake UI or TSF candidate window)
    /// Contains list of candidates with numbered selection (1-5 or more)
    ShowCandidates {
        /// List of candidates (includes display text and actual value)
        candidates: Vec<crate::pipeline::Candidate>,
        /// Current input/syllable being typed
        input: String,
    },
    
    /// Hide candidates UI
    HideCandidates,
}

/// Character information for lookup
#[derive(Debug, Clone, Copy)]
pub struct CharInfo {
    /// Vowel index (0 = not a vowel, 1-6 for a,e,i,o,u,y)
    pub vowel_index: u8,

    /// Macro index (for special transformations)
    pub macro_index: u8,

    /// Double character index
    pub double_char_index: u8,

    /// Tone index
    pub tone_index: u8,

    /// Current tone
    pub current_tone: u8,

    /// Is breve mark
    pub is_breve: bool,

    /// Is separator (space, newline, etc.)
    pub is_separator: bool,

    /// Is soft separator (punctuation)
    pub is_soft_separator: bool,

    /// VNI double index
    pub vni_double_index: u8,

    /// Word form
    pub word_form: WordForm,

    /// Consonant 1 offset
    pub c1_offset: Option<usize>,

    /// Vowel offset
    pub v_offset: Option<usize>,

    /// Consonant 2 offset
    pub c2_offset: Option<usize>,
}

impl Default for CharInfo {
    fn default() -> Self {
        Self {
            vowel_index: 0,
            macro_index: 0,
            double_char_index: 0,
            tone_index: 0,
            current_tone: 0,
            is_breve: false,
            is_separator: false,
            is_soft_separator: false,
            vni_double_index: 0,
            word_form: WordForm::Empty,
            c1_offset: None,
            v_offset: None,
            c2_offset: None,
        }
    }
}

/// Configuration for the IME
#[derive(Debug, Clone)]
pub struct Config {
    /// Enable/disable the IME
    pub enabled: bool,

    /// Auto-commit on separator
    pub auto_commit: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_commit: true,
        }
    }
}

