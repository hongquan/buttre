//! Unicode handling utilities for Vietnamese text

pub mod normalization;
pub mod vowel_sequences;

// Re-export commonly used functions
pub use normalization::{normalize_nfc, normalize_nfd, sanitize_filename, str_eq_normalized};
pub use vowel_sequences::{
    char_to_vnlexi, get_tone_position, lookup_vowel_seq, lookup_vowel_seq_str, VnLexiName,
    VowelSeq, VowelSeqInfo, VSEQ_LIST,
};
