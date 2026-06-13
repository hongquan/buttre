//! Vietnamese Vowel Sequences
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/unicode_vowel_sequences_tests.rs`.
//!
//! This module defines Vietnamese vowel sequence patterns and provides utilities for
//! analyzing and manipulating vowel sequences. It's ported from Unikey's vowel sequence
//! table for optimal Vietnamese text processing.
//!
//! # Overview
//!
//! Vietnamese vowel sequences define how multiple vowels combine and how tones are positioned
//! within those combinations. This module provides:
//!
//! - `VnLexiName`: Enumeration of Vietnamese lexical characters
//! - `VowelSeq`: Enumeration of vowel sequence identifiers
//! - `VowelSeqInfo`: Structural information about each vowel sequence
//! - `VSEQ_LIST`: Complete table of 70 vowel sequence patterns
//! - Helper functions for vowel sequence analysis and lookup
//!
//! # Data Source
//!
//! Ported from: `.reference/unikey/x-unikey/src/ukengine/ukengine.cpp` (lines 77-187)

/// Vietnamese lexical character names
///
/// Defines single-character units in Vietnamese, used to identify vowel patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VnLexiName {
    /// Non-Vietnamese character (sentinel)
    NonVnChar = 0,
    /// 'a'
    A = 1,
    /// 'ă' (a with breve)
    Ar = 2,
    /// 'â' (a with circumflex)
    Ab = 3,
    /// 'e'
    E = 4,
    /// 'ê' (e with circumflex)
    Er = 5,
    /// 'i'
    I = 6,
    /// 'o'
    O = 7,
    /// 'ô' (o with circumflex)
    Or = 8,
    /// 'ơ' (o with horn)
    Oh = 9,
    /// 'u'
    U = 10,
    /// 'ư' (u with horn)
    Uh = 11,
    /// 'y'
    Y = 12,
    /// 'b'
    B = 13,
    /// 'c'
    C = 14,
    /// 'ch'
    Ch = 15,
    /// 'd'
    D = 16,
    /// 'đ' (d with stroke)
    Dd = 17,
    /// 'dz'
    Dz = 18,
    /// 'g'
    G = 19,
    /// 'gh'
    Gh = 20,
    /// 'gi'
    Gi = 21,
    /// 'gin'
    Gin = 22,
    /// 'k'
    K = 23,
    /// 'kh'
    Kh = 24,
    /// 'l'
    L = 25,
    /// 'm'
    M = 26,
    /// 'n'
    N = 27,
    /// 'ng'
    Ng = 28,
    /// 'ngh'
    Ngh = 29,
    /// 'nh'
    Nh = 30,
    /// 'p'
    P = 31,
    /// 'ph'
    Ph = 32,
    /// 'q'
    Q = 33,
    /// 'qu'
    Qu = 34,
    /// 'r'
    R = 35,
    /// 's'
    S = 36,
    /// 't'
    T = 37,
    /// 'th'
    Th = 38,
    /// 'tr'
    Tr = 39,
    /// 'v'
    V = 40,
    /// 'x'
    X = 41,
}

/// Vowel sequence identifiers
///
/// Represents distinct vowel sequences found in Vietnamese syllables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VowelSeq {
    /// Nil/None
    Nil = 0,
    /// 'a'
    A = 1,
    /// 'ă' (a with breve)
    Ar = 2,
    /// 'â' (a with circumflex)
    Ab = 3,
    /// 'e'
    E = 4,
    /// 'ê' (e with circumflex)
    Er = 5,
    /// 'i'
    I = 6,
    /// 'o'
    O = 7,
    /// 'ô' (o with circumflex)
    Or = 8,
    /// 'ơ' (o with horn)
    Oh = 9,
    /// 'u'
    U = 10,
    /// 'ư' (u with horn)
    Uh = 11,
    /// 'y'
    Y = 12,
    /// 'ai'
    Ai = 13,
    /// 'ao'
    Ao = 14,
    /// 'au'
    Au = 15,
    /// 'ay'
    Ay = 16,
    /// 'aru' (ău)
    Aru = 17,
    /// 'ary' (ăy)
    Ary = 18,
    /// 'eo'
    Eo = 19,
    /// 'eu'
    Eu = 20,
    /// 'eru' (êu)
    Eru = 21,
    /// 'ia'
    Ia = 22,
    /// 'ie'
    Ie = 23,
    /// 'ier' (iê)
    Ier = 24,
    /// 'ieu' (iêu)
    Ieu = 25,
    /// 'iu'
    Iu = 26,
    /// 'oa'
    Oa = 27,
    /// 'oab'
    Oab = 28,
    /// 'oe'
    Oe = 29,
    /// 'oai'
    Oai = 30,
    /// 'oay'
    Oay = 31,
    /// 'oeo'
    Oeo = 32,
    /// 'oi'
    Oi = 33,
    /// 'ori' (ôi)
    Ori = 34,
    /// 'ohi' (ơi)
    Ohi = 35,
    /// 'ua'
    Ua = 36,
    /// 'uar' (uă)
    Uar = 37,
    /// 'uary' (uăy)
    Uary = 38,
    /// 'ue'
    Ue = 39,
    /// 'uer' (uê)
    Uer = 40,
    /// 'ui'
    Ui = 41,
    /// 'uhi' (uư+i)
    Uhi = 42,
    /// 'uo'
    Uo = 43,
    /// 'uoi'
    Uoi = 44,
    /// 'uor' (uô)
    Uor = 45,
    /// 'uori' (uôi)
    Uori = 46,
    /// 'uohi' (uơi)
    Uohi = 47,
    /// 'uoh' (uơ)
    Uoh = 48,
    /// 'uohu' (uơu)
    Uohu = 49,
    /// 'uou'
    Uou = 50,
    /// 'uhou' (uưu)
    Uhou = 51,
    /// 'uu'
    Uu = 52,
    /// 'uhu' (uư)
    Uhu = 53,
    /// 'uy'
    Uy = 54,
    /// 'uya'
    Uya = 55,
    /// 'uye'
    Uye = 56,
    /// 'uyer' (uyê)
    Uyer = 57,
    /// 'uyu'
    Uyu = 58,
    /// 'uha' (uưa)
    Uha = 59,
    /// 'uho' (uưo)
    Uho = 60,
    /// 'uhoi' (uưoi)
    Uhoi = 61,
    /// 'uhoh' (uươ)
    Uhoh = 62,
    /// 'uhohi' (uươi)
    Uhohi = 63,
    /// 'uhohu' (uươu)
    Uhohu = 64,
    /// 'ye'
    Ye = 65,
    /// 'yer' (yê)
    Yer = 66,
    /// 'yeu'
    Yeu = 67,
    /// 'yeru' (yêu)
    Yeru = 68,
}

/// Information about a vowel sequence pattern
///
/// Describes the structure and properties of a vowel sequence, including:
/// - Length and completeness
/// - Component vowels
/// - Tone positioning rules
/// - Modifier combinations (roof/circumflex, hook)
#[derive(Debug, Clone, Copy)]
pub struct VowelSeqInfo {
    /// Number of characters in sequence (1-3)
    pub len: u8,
    /// Completeness flag (whether sequence is complete/valid)
    pub complete: u8,
    /// Whether consonant suffix is allowed
    pub con_suffix: u8,
    /// Component vowels [0]=first, [1]=second, [2]=third
    pub vowels: [VnLexiName; 3],
    /// Corresponding vowel sequences [0]=first, [1]=second, [2]=third
    pub sub_seqs: [VowelSeq; 3],
    /// Position of tone mark for roof (circumflex) variant (-1=none, 0=first, 1=second, etc.)
    pub roof_pos: i8,
    /// Vowel sequence with roof (circumflex)
    pub with_roof: VowelSeq,
    /// Position of tone mark for hook (horn) variant (-1=none, 0=first, 1=second, etc.)
    pub hook_pos: i8,
    /// Vowel sequence with hook (horn)
    pub with_hook: VowelSeq,
}

/// Complete table of 70 Vietnamese vowel sequence patterns
///
/// This is the main lookup table for vowel sequence analysis, ported from Unikey.
/// Each entry describes a vowel sequence pattern, how it can be modified with
/// diacritical marks, and where tones should be positioned.
pub const VSEQ_LIST: &[VowelSeqInfo] = &[
    // Single vowels (indices 0-11)
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::A, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::A, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Ar, hook_pos: -1, with_hook: VowelSeq::Ab },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::Ar, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Ar, VowelSeq::Nil, VowelSeq::Nil], roof_pos: 0, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Ab },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::Ab, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Ab, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Ar, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::E, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::E, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Er, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::Er, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Er, VowelSeq::Nil, VowelSeq::Nil], roof_pos: 0, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::I, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::I, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::O, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::O, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Or, hook_pos: -1, with_hook: VowelSeq::Oh },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::Or, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Or, VowelSeq::Nil, VowelSeq::Nil], roof_pos: 0, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Oh },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::Oh, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Oh, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Or, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Uh },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::Uh, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Uh, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 1, complete: 1, con_suffix: 1, vowels: [VnLexiName::Y, VnLexiName::NonVnChar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Y, VowelSeq::Nil, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    // Two-vowel sequences (indices 12-48)
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::A, VnLexiName::I, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::A, VowelSeq::Ai, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::A, VnLexiName::O, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::A, VowelSeq::Ao, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::A, VnLexiName::U, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::A, VowelSeq::Au, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Aru, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::A, VnLexiName::Y, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::A, VowelSeq::Ay, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Ary, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Ar, VnLexiName::U, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Ar, VowelSeq::Aru, VowelSeq::Nil], roof_pos: 0, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Ar, VnLexiName::Y, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Ar, VowelSeq::Ary, VowelSeq::Nil], roof_pos: 0, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::E, VnLexiName::O, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::E, VowelSeq::Eo, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 0, con_suffix: 0, vowels: [VnLexiName::E, VnLexiName::U, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::E, VowelSeq::Eu, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Eru, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Er, VnLexiName::U, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Er, VowelSeq::Eru, VowelSeq::Nil], roof_pos: 0, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::I, VnLexiName::A, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::I, VowelSeq::Ia, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 0, con_suffix: 1, vowels: [VnLexiName::I, VnLexiName::E, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::I, VowelSeq::Ie, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Ier, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::I, VnLexiName::Er, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::I, VowelSeq::Ier, VowelSeq::Nil], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::I, VnLexiName::U, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::I, VowelSeq::Iu, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::O, VnLexiName::A, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::O, VowelSeq::Oa, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Oab },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::O, VnLexiName::Ab, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::O, VowelSeq::Oab, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::O, VnLexiName::E, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::O, VowelSeq::Oe, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::O, VnLexiName::I, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::O, VowelSeq::Oi, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Ori, hook_pos: -1, with_hook: VowelSeq::Ohi },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Or, VnLexiName::I, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Or, VowelSeq::Ori, VowelSeq::Nil], roof_pos: 0, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Ohi },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Oh, VnLexiName::I, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Oh, VowelSeq::Ohi, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Ori, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::A, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Ua, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Uar, hook_pos: -1, with_hook: VowelSeq::Uha },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::Ar, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Uar, VowelSeq::Nil], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 0, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::E, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Ue, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Uer, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::Er, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Uer, VowelSeq::Nil], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::I, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Ui, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Uhi },
    VowelSeqInfo { len: 2, complete: 0, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::O, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Uo, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Uor, hook_pos: -1, with_hook: VowelSeq::Uho },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::Or, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Uor, VowelSeq::Nil], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Uoh },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::Oh, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Uoh, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Uor, hook_pos: 1, with_hook: VowelSeq::Uhoh },
    VowelSeqInfo { len: 2, complete: 0, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::U, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Uu, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Uhu },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::Y, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::U, VowelSeq::Uy, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Uh, VnLexiName::A, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Uh, VowelSeq::Uha, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Uh, VnLexiName::I, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Uh, VowelSeq::Uhi, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 0, con_suffix: 1, vowels: [VnLexiName::Uh, VnLexiName::O, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Uh, VowelSeq::Uho, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Uhoh },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::Uh, VnLexiName::Oh, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Uh, VowelSeq::Uhoh, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 0, vowels: [VnLexiName::Uh, VnLexiName::U, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Uh, VowelSeq::Uhu, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 0, con_suffix: 1, vowels: [VnLexiName::Y, VnLexiName::E, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Y, VowelSeq::Ye, VowelSeq::Nil], roof_pos: -1, with_roof: VowelSeq::Yer, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 2, complete: 1, con_suffix: 1, vowels: [VnLexiName::Y, VnLexiName::Er, VnLexiName::NonVnChar], sub_seqs: [VowelSeq::Y, VowelSeq::Yer, VowelSeq::Nil], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    // Three-vowel sequences (indices 49-69)
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::I, VnLexiName::E, VnLexiName::U], sub_seqs: [VowelSeq::I, VowelSeq::Ie, VowelSeq::Ieu], roof_pos: -1, with_roof: VowelSeq::Ieu, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::I, VnLexiName::Er, VnLexiName::U], sub_seqs: [VowelSeq::I, VowelSeq::Ier, VowelSeq::Ieu], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::O, VnLexiName::A, VnLexiName::I], sub_seqs: [VowelSeq::O, VowelSeq::Oa, VowelSeq::Oai], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::O, VnLexiName::A, VnLexiName::Y], sub_seqs: [VowelSeq::O, VowelSeq::Oa, VowelSeq::Oay], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::O, VnLexiName::E, VnLexiName::O], sub_seqs: [VowelSeq::O, VowelSeq::Oe, VowelSeq::Oeo], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::A, VnLexiName::Y], sub_seqs: [VowelSeq::U, VowelSeq::Ua, VowelSeq::Ay], roof_pos: -1, with_roof: VowelSeq::Uary, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::Ar, VnLexiName::Y], sub_seqs: [VowelSeq::U, VowelSeq::Uar, VowelSeq::Uary], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::O, VnLexiName::I], sub_seqs: [VowelSeq::U, VowelSeq::Uo, VowelSeq::Uoi], roof_pos: -1, with_roof: VowelSeq::Uori, hook_pos: -1, with_hook: VowelSeq::Uhoi },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::O, VnLexiName::U], sub_seqs: [VowelSeq::U, VowelSeq::Uo, VowelSeq::Uou], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Uhou },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::Or, VnLexiName::I], sub_seqs: [VowelSeq::U, VowelSeq::Uor, VowelSeq::Uori], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Uohi },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::Oh, VnLexiName::I], sub_seqs: [VowelSeq::U, VowelSeq::Uoh, VowelSeq::Uohi], roof_pos: -1, with_roof: VowelSeq::Uori, hook_pos: 1, with_hook: VowelSeq::Uhohi },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::Oh, VnLexiName::U], sub_seqs: [VowelSeq::U, VowelSeq::Uoh, VowelSeq::Uohu], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 1, with_hook: VowelSeq::Uhohu },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::Y, VnLexiName::A], sub_seqs: [VowelSeq::U, VowelSeq::Uy, VowelSeq::Uya], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::Y, VnLexiName::E], sub_seqs: [VowelSeq::U, VowelSeq::Uy, VowelSeq::Uye], roof_pos: -1, with_roof: VowelSeq::Uyer, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 1, vowels: [VnLexiName::U, VnLexiName::Y, VnLexiName::Er], sub_seqs: [VowelSeq::U, VowelSeq::Uy, VowelSeq::Uyer], roof_pos: 2, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::U, VnLexiName::Y, VnLexiName::U], sub_seqs: [VowelSeq::U, VowelSeq::Uy, VowelSeq::Uyu], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::Uh, VnLexiName::O, VnLexiName::I], sub_seqs: [VowelSeq::Uh, VowelSeq::Uho, VowelSeq::Uhoi], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Uhohi },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::Uh, VnLexiName::O, VnLexiName::U], sub_seqs: [VowelSeq::Uh, VowelSeq::Uho, VowelSeq::Uhou], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Uhohu },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::Uh, VnLexiName::Oh, VnLexiName::I], sub_seqs: [VowelSeq::Uh, VowelSeq::Uhoh, VowelSeq::Uhohi], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::Uh, VnLexiName::Oh, VnLexiName::U], sub_seqs: [VowelSeq::Uh, VowelSeq::Uhoh, VowelSeq::Uhohu], roof_pos: -1, with_roof: VowelSeq::Nil, hook_pos: 0, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 0, con_suffix: 0, vowels: [VnLexiName::Y, VnLexiName::E, VnLexiName::U], sub_seqs: [VowelSeq::Y, VowelSeq::Ye, VowelSeq::Yeu], roof_pos: -1, with_roof: VowelSeq::Yeru, hook_pos: -1, with_hook: VowelSeq::Nil },
    VowelSeqInfo { len: 3, complete: 1, con_suffix: 0, vowels: [VnLexiName::Y, VnLexiName::Er, VnLexiName::U], sub_seqs: [VowelSeq::Y, VowelSeq::Yer, VowelSeq::Yeru], roof_pos: 1, with_roof: VowelSeq::Nil, hook_pos: -1, with_hook: VowelSeq::Nil },
];

/// Convert a character to its VnLexiName equivalent
///
/// # Arguments
/// * `c` - Character to convert
///
/// # Returns
/// The corresponding VnLexiName, or NonVnChar if not a Vietnamese character
///
/// # Examples
/// ```
/// use buttre_engine::unicode::vowel_sequences::char_to_vnlexi;
/// use buttre_engine::unicode::VnLexiName;
/// assert_eq!(char_to_vnlexi('a'), VnLexiName::A);
/// assert_eq!(char_to_vnlexi('z'), VnLexiName::NonVnChar);
/// ```
pub fn char_to_vnlexi(c: char) -> VnLexiName {
    let lower = c.to_lowercase().next().unwrap_or(c);
    match lower {
        // Base vowels
        'a' => VnLexiName::A,
        'â' => VnLexiName::Ar,
        'ă' => VnLexiName::Ab,
        'e' => VnLexiName::E,
        'ê' => VnLexiName::Er,
        'i' => VnLexiName::I,
        'o' => VnLexiName::O,
        'ô' => VnLexiName::Or,
        'ơ' => VnLexiName::Oh,
        'u' => VnLexiName::U,
        'ư' => VnLexiName::Uh,
        'y' => VnLexiName::Y,
        
        // Consonants (only single-character consonants for this simple mapping)
        'b' => VnLexiName::B,
        'c' => VnLexiName::C,
        'd' => VnLexiName::D,
        'đ' => VnLexiName::Dd,
        'g' => VnLexiName::G,
        'k' => VnLexiName::K,
        'l' => VnLexiName::L,
        'm' => VnLexiName::M,
        'n' => VnLexiName::N,
        'p' => VnLexiName::P,
        'q' => VnLexiName::Q,
        'r' => VnLexiName::R,
        's' => VnLexiName::S,
        't' => VnLexiName::T,
        'v' => VnLexiName::V,
        'x' => VnLexiName::X,
        
        _ => VnLexiName::NonVnChar,
    }
}

/// Look up a vowel sequence by its vowel pattern
///
/// Searches the VSEQ_LIST for a sequence matching the given vowel pattern
/// (up to 3 vowels long).
///
/// # Arguments
/// * `vowels` - Array of VnLexiName values (up to 3, pad with NonVnChar)
/// * `len` - Number of vowels to match (1-3)
///
/// # Returns
/// The matching VowelSeqInfo, or None if no match found
///
/// # Examples
/// ```
/// use buttre_engine::unicode::vowel_sequences::{lookup_vowel_seq, VnLexiName};
/// let vowels = [VnLexiName::A, VnLexiName::I, VnLexiName::NonVnChar];
/// let info = lookup_vowel_seq(&vowels, 2).unwrap();
/// assert_eq!(info.len, 2); // "ai" is 2 characters
/// ```
pub fn lookup_vowel_seq(vowels: &[VnLexiName; 3], len: usize) -> Option<&'static VowelSeqInfo> {
    if len < 1 || len > 3 {
        return None;
    }

    VSEQ_LIST
        .iter()
        .find(|seq| {
            seq.len as usize == len
                && seq.vowels[0] == vowels[0]
                && seq.vowels[1] == vowels[1]
                && seq.vowels[2] == vowels[2]
        })
}

/// Look up a vowel sequence by string pattern
///
/// Convenience function that converts a string of Vietnamese vowels to VnLexiName
/// and looks up the sequence.
///
/// # Arguments
/// * `pattern` - String like "ai", "oa", "iêu", etc.
///
/// # Returns
/// The matching VowelSeqInfo, or None if no match found
///
/// # Examples
/// ```
/// use buttre_engine::unicode::vowel_sequences::lookup_vowel_seq_str;
/// let info = lookup_vowel_seq_str("ai").unwrap();
/// assert_eq!(info.len, 2);
/// ```
pub fn lookup_vowel_seq_str(pattern: &str) -> Option<&'static VowelSeqInfo> {
    let chars: Vec<char> = pattern.chars().collect();
    if chars.is_empty() || chars.len() > 3 {
        return None;
    }

    let mut vowels = [VnLexiName::NonVnChar; 3];
    for (i, c) in chars.iter().enumerate() {
        vowels[i] = char_to_vnlexi(*c);
    }

    lookup_vowel_seq(&vowels, chars.len())
}

/// Get the tone position for a vowel sequence
///
/// Determines where the tone mark should be placed within a vowel sequence.
/// This is crucial for correct Vietnamese diacritical placement.
///
/// # Arguments
/// * `seq_info` - The VowelSeqInfo for the sequence
/// * `has_final_consonant` - Whether the syllable has a final consonant
///
/// # Returns
/// The position of the tone (0 = first vowel, 1 = second vowel, 2 = third vowel, or -1 if unknown)
///
/// # Examples
/// ```
/// use buttre_engine::unicode::vowel_sequences::{lookup_vowel_seq_str, get_tone_position};
/// let info = lookup_vowel_seq_str("ai").unwrap();
/// let pos = get_tone_position(info, false);
/// assert_eq!(pos, 0); // Tone goes on 'a'
/// ```
pub fn get_tone_position(seq_info: &VowelSeqInfo, has_final_consonant: bool) -> i8 {
    // This function matches Unikey's getTonePosition logic exactly
    // Source: .reference/unikey/x-unikey/src/ukengine/ukengine.cpp:929-950
    
    // Single vowel: always position 0
    if seq_info.len == 1 {
        return 0;
    }

    // If has roof position, tone always goes there
    // Example: iê (iér) has roof_pos=1, tone always on ê
    if seq_info.roof_pos != -1 {
        return seq_info.roof_pos;
    }
    
    // If has hook position, tone goes there (with special cases)
    // Example: ươ (uhoh) has hook_pos=0, but special case → tone on ơ (pos 1)
    if seq_info.hook_pos != -1 {
        // Special cases: ươ, ươi, ươu (uhoh, uhohi, uhohu)
        // These sequences have hook_pos=0 but tone should be on middle vowel (pos 1)
        // Check by vowel components: Uh + Oh + (nil/I/U)
        let is_uhoh_family = seq_info.len >= 2 
            && seq_info.vowels[0] == VnLexiName::Uh 
            && seq_info.vowels[1] == VnLexiName::Oh;
        
        if is_uhoh_family {
            return 1; // ươ, ươi, ươu: tone on ơ (middle position)
        }
        
        // For other sequences with hook_pos, use the hook position
        return seq_info.hook_pos;
    }

    // For triple vowels: tone on middle vowel (position 1)
    // This covers patterns like iêu, oai, uây where 3rd vowel is semi-vowel
    if seq_info.len == 3 {
        return 1;
    }

    // Modern style exception for oa, oe, uy
    // In modern Vietnamese orthography, these always have tone on 2nd vowel
    // Examples: "hoá" (modern) vs "hóa" (traditional)
    //           "toè" (modern) vs "tóe" (traditional)  
    //           "huyỷ" (modern) vs "húy" (traditional)
    //
    // We check by looking at the vowel components
    let is_oa = seq_info.vowels[0] == VnLexiName::O && seq_info.vowels[1] == VnLexiName::A;
    let is_oe = seq_info.vowels[0] == VnLexiName::O && seq_info.vowels[1] == VnLexiName::E;
    let is_uy = seq_info.vowels[0] == VnLexiName::U && seq_info.vowels[1] == VnLexiName::Y;
    
    // Modern style is the default for buttre (matching Unikey default)
    let modern_style = true;
    if modern_style && (is_oa || is_oe || is_uy) {
        return 1; // Always tone on 2nd vowel for oa/oe/uy in modern style
    }

    // For remaining double vowels, use standard rule:
    // - If has final consonant: tone on 2nd vowel (pos 1)
    // - If no final consonant: tone on 1st vowel (pos 0)
    //
    // Examples:
    // - "mua" (no final) → "múa" (tone on u = pos 0)
    // - "muat" (has final t) → "muát" (tone on a = pos 1)
    if has_final_consonant {
        1
    } else {
        0
    }
}

