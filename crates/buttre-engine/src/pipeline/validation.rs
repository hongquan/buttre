//! Vietnamese Syllable Structure Parser
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/pipeline_validation_tests.rs`.
//!
//! Parses Vietnamese syllables into components: Onset, Nucleus, Coda
//!
//! ## Vietnamese Syllable Structure
//!
//! Vietnamese syllables follow the pattern: (C₁)V(C₂)
//! - C₁: Optional initial consonant or consonant cluster
//! - V: Required vowel nucleus (single or cluster)
//! - C₂: Optional final consonant
//!
//! ## Examples
//!
//! - "a" → Onset: "", Nucleus: "a", Coda: ""
//! - "ba" → Onset: "b", Nucleus: "a", Coda: ""
//! - "ban" → Onset: "b", Nucleus: "a", Coda: "n"
//! - "thường" → Onset: "th", Nucleus: "ườ", Coda: "ng"

/// Vietnamese syllable structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyllableStructure {
    /// Initial consonant(s): "", "b", "tr", "ngh"
    pub onset: String,
    
    /// Vowel nucleus: "a", "oa", "uye"
    pub nucleus: String,
    
    /// Final consonant: "", "n", "ng", "ch"
    pub coda: String,
}

impl SyllableStructure {
    /// Parse a Vietnamese syllable into components
    ///
    /// ## Algorithm
    ///
    /// 1. Normalize Vietnamese characters to base form (remove tones)
    /// 2. Extract onset (longest matching consonant cluster from start)
    /// 3. Extract coda (longest matching final consonant from end)
    /// 4. Remaining middle part is nucleus
    ///
    /// ## Example
    ///
    /// ```
    /// use buttre_engine::pipeline::validation::SyllableStructure;
    ///
    /// let structure = SyllableStructure::parse("thường");
    /// assert_eq!(structure.onset, "th");
    /// assert_eq!(structure.nucleus, "ươ");
    /// assert_eq!(structure.coda, "ng");
    /// ```
    pub fn parse(syllable: &str) -> Self {
        // Algorithm Step 0: Normalize to lowercase and remove tones
        let syllable_normalized = normalize_vietnamese(syllable);
        
        // Algorithm Step 1: Extract onset (initial consonant cluster)
        let onset = extract_onset(&syllable_normalized);
        let after_onset = &syllable_normalized[onset.len()..];
        
        // Algorithm Step 2: Extract coda (final consonant)
        let coda = extract_coda(after_onset);
        let nucleus_end = after_onset.len() - coda.len();
        let nucleus = &after_onset[..nucleus_end];
        
        Self {
            onset: onset.to_string(),
            nucleus: nucleus.to_string(),
            coda: coda.to_string(),
        }
    }
    
    /// Check if this syllable structure is valid Vietnamese
    ///
    /// ## Algorithm
    ///
    /// Validates:
    /// 1. Onset is in valid onset list
    /// 2. Nucleus is in valid nucleus list
    /// 3. Coda is in valid coda list
    /// 4. Onset-Nucleus-Coda combination is valid
    pub fn is_valid(&self) -> bool {
        self.is_valid_onset() && 
        self.is_valid_nucleus() && 
        self.is_valid_coda() &&
        self.is_valid_combination()
    }
    
    /// Check if onset is valid
    fn is_valid_onset(&self) -> bool {
        VALID_ONSETS.contains(&self.onset.as_str())
    }
    
    /// Check if nucleus is valid
    fn is_valid_nucleus(&self) -> bool {
        // Empty nucleus is invalid
        if self.nucleus.is_empty() {
            return false;
        }
        VALID_NUCLEI.contains(&self.nucleus.as_str())
    }
    
    /// Check if coda is valid
    fn is_valid_coda(&self) -> bool {
        VALID_CODAS.contains(&self.coda.as_str())
    }
    
    /// Check if onset-nucleus-coda combination is valid
    ///
    /// Some combinations are invalid in Vietnamese:
    /// - "iê" + "p/c" → invalid (but "iê" + "t" is valid: việt, tiết)
    fn is_valid_combination(&self) -> bool {
        // Check invalid nucleus-coda combinations
        match (self.nucleus.as_str(), self.coda.as_str()) {
            // "iê" cannot have "p" or "c" codas (but "t" is ok: việt, tiết)
            ("iê", "p" | "c") => false,
            
            // "uơ" and "ưu" only appear in open syllables (no coda)
            ("uơ" | "ưu", coda) if !coda.is_empty() => false,
            
            // All other combinations are valid
            // Note: "ươ" + "ng" is valid (thường, lường, etc.)
            // "ương" is a different nucleus entirely
            _ => true,
        }
    }
}

/// Normalize Vietnamese text to base form (remove tone marks)
///
/// ## Algorithm
///
/// Converts Vietnamese characters with tones to their base forms:
/// - á, à, ả, ã, ạ → a
/// - ế, ề, ể, ễ, ệ → ê
/// - etc.
///
/// This allows syllable structure parsing to work with toned text.
pub fn normalize_vietnamese(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| match c {
            // a variants
            'á' | 'à' | 'ả' | 'ã' | 'ạ' => 'a',
            'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' => 'ă',
            'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => 'â',
            
            // e variants
            'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' => 'e',
            'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => 'ê',
            
            // i variants
            'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => 'i',
            
            // o variants
            'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' => 'o',
            'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' => 'ô',
            'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'ơ',
            
            // u variants
            'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' => 'u',
            'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => 'ư',
            
            // y variants
            'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
            
            // đ
            'đ' => 'đ',
            
            // Keep everything else
            other => other,
        })
        .collect()
}

/// Extract onset (initial consonant cluster) from syllable
///
/// ## Algorithm
///
/// Try to match longest valid onset from the start of syllable.
/// Returns the matched onset string.
pub fn extract_onset(syllable: &str) -> &str {
    // Try 3-char onsets first (longest)
    for &onset in VALID_ONSETS_3CHAR {
        if syllable.starts_with(onset) {
            return onset;
        }
    }
    
    // Try 2-char onsets
    for &onset in VALID_ONSETS_2CHAR {
        if syllable.starts_with(onset) {
            return onset;
        }
    }
    
    // Try 1-char onsets
    for &onset in VALID_ONSETS_1CHAR {
        if syllable.starts_with(onset) {
            return onset;
        }
    }
    
    // No onset (vowel-initial syllable)
    ""
}

/// Extract coda (final consonant) from remaining syllable
///
/// ## Algorithm
///
/// Try to match longest valid coda from the end of syllable.
/// Returns the matched coda string.
pub fn extract_coda(remaining: &str) -> &str {
    // Try 2-char codas first (longest)
    for &coda in VALID_CODAS_2CHAR {
        if remaining.ends_with(coda) {
            return coda;
        }
    }
    
    // Try 1-char codas
    for &coda in VALID_CODAS_1CHAR {
        if remaining.ends_with(coda) {
            return coda;
        }
    }
    
    // No coda (open syllable)
    ""
}

// Vietnamese Phonology Constants

/// Valid 3-character onsets
const VALID_ONSETS_3CHAR: &[&str] = &[
    "ngh", // nghệ, nghĩa
];

/// Valid 2-character onsets
const VALID_ONSETS_2CHAR: &[&str] = &[
    "ch", "gh", "gi", "kh", "ng", "nh", "ph", "qu", "th", "tr",
];

/// Valid 1-character onsets
const VALID_ONSETS_1CHAR: &[&str] = &[
    "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "r", "s", "t", "v", "x",
];

/// All valid onsets (including empty)
const VALID_ONSETS: &[&str] = &[
    "", // Empty onset (vowel-initial)
    // 1-char
    "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "r", "s", "t", "v", "x",
    // 2-char
    "ch", "gh", "gi", "kh", "ng", "nh", "ph", "qu", "th", "tr",
    // 3-char
    "ngh",
];

/// Valid 2-character codas
const VALID_CODAS_2CHAR: &[&str] = &[
    "ch", "ng", "nh",
];

/// Valid 1-character codas
const VALID_CODAS_1CHAR: &[&str] = &[
    "c", "m", "n", "p", "t",
];

/// All valid codas (including empty)
const VALID_CODAS: &[&str] = &[
    "", // Empty coda (open syllable)
    // 1-char
    "c", "m", "n", "p", "t",
    // 2-char
    "ch", "ng", "nh",
];

/// Valid vowel nuclei
const VALID_NUCLEI: &[&str] = &[
    // Single vowels
    "a", "ă", "â", "e", "ê", "i", "o", "ô", "ơ", "u", "ư", "y",
    
    // Diphthongs (2 vowels)
    "ai", "ao", "au", "ay", "âu", "ây",
    "eo", "êu",
    "ia", "iê", "iu",
    "oa", "oă", "oe", "oi", "ôi", "ơi",
    "ua", "uâ", "ue", "ui", "uo", "uy", "uô", "uơ", "ươ",
    "ưa", "ưi", "ưu",
    
    // Triphthongs (3 vowels)
    "iêu",
    "oai", "oao", "oay", "oeo",
    "uao", "uây", "uôi", "ươi", "ươu",
    "uyê",
];

