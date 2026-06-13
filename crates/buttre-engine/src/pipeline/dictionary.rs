//! Dictionary Provider Trait
//!
//! Abstraction for dictionary lookup to avoid tight coupling with specific
//! dictionary implementations (e.g., buttre-nom).

use crate::pipeline::{Candidate, CandidateType};

/// Dictionary provider trait
///
/// Implement this trait to provide dictionary lookup functionality
/// to Stage 8 (Dictionary Lookup).
///
/// ## Example
///
/// ```rust,ignore
/// use buttre_core::engine::pipeline::dictionary::DictionaryProvider;
/// use buttre_core::engine::pipeline::{Candidate, CandidateType};
///
/// struct MyDictionary {
///     // ... implementation
/// }
///
/// impl DictionaryProvider for MyDictionary {
///     fn lookup(&self, keyword: &str) -> Vec<Candidate> {
///         // Query database, return candidates
///         vec![]
///     }
/// }
/// ```
pub trait DictionaryProvider: Send + Sync {
    /// Lookup candidates by keyword (phonetic input)
    ///
    /// ## Arguments
    ///
    /// * `keyword` - The phonetic keyword to search for (e.g., "nguoi")
    ///
    /// ## Returns
    ///
    /// Vector of candidates, sorted by relevance (highest score first)
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let candidates = dict.lookup("nguoi");
    /// // Returns: [Candidate { text: "người", score: 1.0, ... }, ...]
    /// ```
    fn lookup(&self, keyword: &str) -> Vec<Candidate>;

    /// Check if dictionary contains a keyword
    ///
    /// ## Arguments
    ///
    /// * `keyword` - The keyword to check
    ///
    /// ## Returns
    ///
    /// `true` if the keyword exists in the dictionary
    fn contains(&self, keyword: &str) -> bool {
        !self.lookup(keyword).is_empty()
    }

    /// Get total number of entries in dictionary
    ///
    /// ## Returns
    ///
    /// Total count of dictionary entries
    fn count(&self) -> usize {
        0 // Default implementation
    }
}

/// Simple in-memory dictionary for testing
///
/// This is a minimal implementation for testing purposes.
/// Use a real database-backed dictionary in production.
#[derive(Debug, Clone, Default)]
pub struct SimpleDictionary {
    /// In-memory entries: keyword -> candidates
    entries: std::collections::HashMap<String, Vec<Candidate>>,
}

impl SimpleDictionary {
    /// Create a new empty dictionary
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }

    /// Add a candidate to the dictionary
    ///
    /// ## Arguments
    ///
    /// * `keyword` - The phonetic keyword (e.g., "nguoi")
    /// * `text` - The candidate text (e.g., "người")
    /// * `candidate_type` - Type of candidate
    /// * `score` - Relevance score
    pub fn add(&mut self, keyword: &str, text: &str, candidate_type: CandidateType, score: f32) {
        let candidate = Candidate {
            text: text.to_string(),
            value: None, // For simple dictionary, text == value
            candidate_type,
            score,
        };

        self.entries
            .entry(keyword.to_string())
            .or_insert_with(Vec::new)
            .push(candidate);
    }

    /// Add multiple candidates for a keyword
    pub fn add_candidates(&mut self, keyword: &str, candidates: Vec<Candidate>) {
        self.entries
            .entry(keyword.to_string())
            .or_insert_with(Vec::new)
            .extend(candidates);
    }
}

impl DictionaryProvider for SimpleDictionary {
    fn lookup(&self, keyword: &str) -> Vec<Candidate> {
        self.entries
            .get(keyword)
            .cloned()
            .unwrap_or_default()
    }

    fn count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dictionary() {
        let mut dict = SimpleDictionary::new();

        // Add candidates
        dict.add("nguoi", "người", CandidateType::Vietnamese, 1.0);
        dict.add("nguoi", "𠊛", CandidateType::Nom, 0.5);

        // Lookup
        let candidates = dict.lookup("nguoi");
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].text, "người");
        assert_eq!(candidates[1].text, "𠊛");

        // Contains
        assert!(dict.contains("nguoi"));
        assert!(!dict.contains("xyz"));

        // Count
        assert_eq!(dict.count(), 1); // 1 unique keyword
    }

    #[test]
    fn test_empty_dictionary() {
        let dict = SimpleDictionary::new();

        assert!(dict.lookup("test").is_empty());
        assert!(!dict.contains("test"));
        assert_eq!(dict.count(), 0);
    }

    #[test]
    fn test_add_candidates() {
        let mut dict = SimpleDictionary::new();

        let candidates = vec![
            Candidate {
                text: "trời".to_string(),
                value: None,
                candidate_type: CandidateType::Vietnamese,
                score: 1.0,
            },
            Candidate {
                text: "𡗶".to_string(),
                value: None,
                candidate_type: CandidateType::Nom,
                score: 0.8,
            },
        ];

        dict.add_candidates("troi", candidates);

        let result = dict.lookup("troi");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].text, "trời");
        assert_eq!(result[1].text, "𡗶");
    }
}
