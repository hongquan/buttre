//! Vowel Cluster Detection
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/vowel_cluster_tests.rs`.
//!
//! This module provides algorithms for detecting vowel clusters in Vietnamese syllables.
//!
//! ## Vowel Cluster
//!
//! A vowel cluster is a sequence of consecutive vowels in a syllable.
//! Examples:
//! - "trường" has cluster "ươ" at positions 2-3
//! - "hoà" has cluster "oà" at positions 1-2
//! - "cười" has cluster "ươi" at positions 1-3
//!
//! ## Algorithm
//!
//! The cluster detection algorithm:
//! 1. Scan the buffer from left to right
//! 2. Identify vowel characters (a, ă, â, e, ê, i, o, ô, ơ, u, ư, y)
//! 3. Group consecutive vowels into clusters
//! 4. Classify each cluster by type (single, double, triple, compound)

/// Vowel Cluster
///
/// Represents a sequence of consecutive vowels in a syllable.
///
/// ## Example
///
/// ```rust,ignore
/// // For the word "trường"
/// VowelCluster {
///     start_pos: 2,
///     end_pos: 4,  // Exclusive
///     vowels: vec!['ư', 'ơ'],
///     cluster_type: ClusterType::CompoundUO,
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VowelCluster {
    /// Start position in the buffer (inclusive)
    pub start_pos: usize,
    
    /// End position in the buffer (exclusive)
    pub end_pos: usize,
    
    /// Vowel characters in the cluster
    pub vowels: Vec<char>,
    
    /// Type of cluster
    pub cluster_type: ClusterType,
}

/// Cluster Type
///
/// Classification of vowel clusters based on their composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterType {
    /// Single vowel (a, e, i, o, u, y, ă, â, ê, ô, ơ, ư)
    Single,
    
    /// Double vowel (ai, ao, oa, oe, etc.)
    Double,
    
    /// Triple vowel (oai, uôi, ươi, etc.)
    Triple,
    
    /// Compound uo/ươ (special handling needed)
    CompoundUO,
    
    /// Double with oa/oe pattern
    DoubleOA,
    
    /// Invalid (shouldn't happen in valid Vietnamese)
    Invalid,
}

impl VowelCluster {
    /// Get the length of the cluster
    pub fn len(&self) -> usize {
        self.vowels.len()
    }
    
    /// Check if the cluster is empty
    pub fn is_empty(&self) -> bool {
        self.vowels.is_empty()
    }
    
    /// Check if a position is within this cluster
    pub fn contains_position(&self, pos: usize) -> bool {
        pos >= self.start_pos && pos < self.end_pos
    }
    
    /// Get the cluster as a string
    pub fn to_string(&self) -> String {
        self.vowels.iter().collect()
    }
}

/// Check if a character is a Vietnamese vowel
///
/// ## Arguments
///
/// - `ch`: Character to check
///
/// ## Returns
///
/// `true` if the character is a Vietnamese vowel (with or without marks)
pub fn is_vowel(ch: char) -> bool {
    matches!(
        ch,
        'a' | 'ă' | 'â' | 'e' | 'ê' | 'i' | 'o' | 'ô' | 'ơ' | 'u' | 'ư' | 'y' |
        'A' | 'Ă' | 'Â' | 'E' | 'Ê' | 'I' | 'O' | 'Ô' | 'Ơ' | 'U' | 'Ư' | 'Y' |
        // With tone marks
        'á' | 'à' | 'ả' | 'ã' | 'ạ' |
        'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' |
        'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' |
        'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' |
        'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' |
        'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' |
        'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' |
        'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' |
        'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' |
        'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' |
        'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' |
        'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' |
        // Uppercase with tones
        'Á' | 'À' | 'Ả' | 'Ã' | 'Ạ' |
        'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' |
        'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' |
        'É' | 'È' | 'Ẻ' | 'Ẽ' | 'Ẹ' |
        'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' |
        'Í' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Ị' |
        'Ó' | 'Ò' | 'Ỏ' | 'Õ' | 'Ọ' |
        'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' |
        'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' |
        'Ú' | 'Ù' | 'Ủ' | 'Ũ' | 'Ụ' |
        'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' |
        'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ'
    )
}

/// Normalize a vowel to its base form (remove tone marks)
///
/// ## Arguments
///
/// - `ch`: Vowel character (with or without tone)
///
/// ## Returns
///
/// The base form of the vowel (without tone marks)
///
/// ## Example
///
/// ```rust,ignore
/// assert_eq!(normalize_vowel('á'), 'a');
/// assert_eq!(normalize_vowel('ề'), 'ê');
/// assert_eq!(normalize_vowel('ờ'), 'ơ');
/// ```
pub fn normalize_vowel(ch: char) -> char {
    match ch {
        // a family
        'á' | 'à' | 'ả' | 'ã' | 'ạ' | 'Á' | 'À' | 'Ả' | 'Ã' | 'Ạ' => 'a',
        'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' => 'ă',
        'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' => 'â',
        
        // e family
        'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' | 'É' | 'È' | 'Ẻ' | 'Ẽ' | 'Ẹ' => 'e',
        'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' => 'ê',
        
        // i family
        'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' | 'Í' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Ị' => 'i',
        
        // o family
        'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' | 'Ó' | 'Ò' | 'Ỏ' | 'Õ' | 'Ọ' => 'o',
        'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' => 'ô',
        'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' | 'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' => 'ơ',
        
        // u family
        'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' | 'Ú' | 'Ù' | 'Ủ' | 'Ũ' | 'Ụ' => 'u',
        'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' | 'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' => 'ư',
        
        // y family
        'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' | 'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'y',
        
        // Already base form or uppercase
        c => c.to_lowercase().next().unwrap_or(c),
    }
}

/// Find all vowel clusters in a buffer
///
/// ## Algorithm
///
/// 1. Scan the buffer character by character
/// 2. When a vowel is found, start a new cluster
/// 3. Continue adding consecutive vowels to the cluster
/// 4. When a non-vowel is found, finalize the cluster
/// 5. Classify the cluster by type
///
/// ## Arguments
///
/// - `buffer`: The text buffer to scan
///
/// ## Returns
///
/// Vector of vowel clusters found in the buffer
///
/// ## Example
///
/// ```rust,ignore
/// let clusters = find_vowel_clusters("trường");
/// // Returns: [VowelCluster { start_pos: 2, end_pos: 4, vowels: ['ư', 'ơ'], ... }]
/// ```
pub fn find_vowel_clusters(buffer: &str) -> Vec<VowelCluster> {
    let chars: Vec<char> = buffer.chars().collect();
    let mut clusters = Vec::new();
    let mut i = 0;
    
    while i < chars.len() {
        if is_vowel(chars[i]) {
            // Start a new cluster
            let start_pos = i;
            let mut vowels = Vec::new();
            
            // Collect consecutive vowels (normalize to base form)
            while i < chars.len() && is_vowel(chars[i]) {
                vowels.push(normalize_vowel(chars[i]));
                i += 1;
            }
            
            let end_pos = i;
            let cluster_type = classify_cluster(&vowels);
            
            clusters.push(VowelCluster {
                start_pos,
                end_pos,
                vowels,
                cluster_type,
            });
        } else {
            i += 1;
        }
    }
    
    clusters
}

/// Classify a vowel cluster by its composition
///
/// ## Algorithm
///
/// 1. Check the number of vowels (1, 2, or 3)
/// 2. For double vowels, check for special patterns:
///    - uo/ươ (compound)
///    - oa/oe (double OA pattern)
/// 3. Return the appropriate ClusterType
///
/// ## Arguments
///
/// - `vowels`: Slice of vowel characters (lowercase)
///
/// ## Returns
///
/// The classified ClusterType
pub fn classify_cluster(vowels: &[char]) -> ClusterType {
    match vowels.len() {
        0 => ClusterType::Invalid,
        1 => ClusterType::Single,
        2 => {
            // Check for special patterns
            if matches!((vowels[0], vowels[1]), ('u', 'o') | ('u', 'ơ') | ('ư', 'o') | ('ư', 'ơ')) {
                ClusterType::CompoundUO
            } else if matches!((vowels[0], vowels[1]), ('o', 'a') | ('o', 'e')) {
                ClusterType::DoubleOA
            } else {
                ClusterType::Double
            }
        },
        3 => ClusterType::Triple,
        _ => ClusterType::Invalid,
    }
}

