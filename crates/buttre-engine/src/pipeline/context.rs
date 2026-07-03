//! Typing Context - State container for pipeline processing
//!
//! The TypingContext holds all state needed during pipeline processing,
//! including buffers, cursor position, history, and temporary flags.
//!
//! ## 7-Stage Pipeline Support
//!
//! This context supports the 7-stage pipeline architecture:
//! - Stage 1: Normalization — normalize input, populate char_buffer
//! - Stage 2: Gatekeeper — route non-Vietnamese / temp-English passthrough
//! - Stage 3: Compose — recompute-from-raw: segment → transform → tone → fallback
//! - Stage 4: Orthography — normalize Unicode form
//! - Stage 5: Learning — track patterns (future, currently no-op)
//! - Stage 6: Lookup — dictionary lookup (Nôm candidates)
//! - Stage 7: Output — diff last_output → syllable_buffer → emit actions

use std::collections::HashMap;

// ========================================
// CharInfo - Character with case tracking
// ========================================

/// Character information with case tracking
///
/// Combines a normalized (lowercase) character with its original case information.
/// This replaces the separate `raw_buffer: String` and `case_mask: Vec<bool>`.
///
/// ## Algorithm
///
/// When a character is pushed to the buffer:
/// 1. Store the normalized (lowercase) version in `ch`
/// 2. Store whether the original was uppercase in `is_uppercase`
///
/// When outputting:
/// 1. If `is_uppercase` is true, convert `ch` to uppercase
/// 2. Otherwise, output `ch` as-is
///
/// ## Example
///
/// ```ignore
/// let info = CharInfo::from('A');
/// assert_eq!(info.ch, 'a');
/// assert_eq!(info.is_uppercase, true);
/// assert_eq!(info.to_output_char(), 'A');
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharInfo {
    /// Normalized (lowercase) character for internal processing
    pub ch: char,
    
    /// Whether the original input was uppercase
    pub is_uppercase: bool,
}

impl CharInfo {
    /// Create a new CharInfo from a character
    ///
    /// Normalizes alphabetic characters to lowercase while preserving case info.
    pub fn new(input: char) -> Self {
        Self {
            ch: input.to_lowercase().next().unwrap_or(input),
            is_uppercase: input.is_uppercase(),
        }
    }
    
    /// Create CharInfo from already-normalized char with explicit case
    pub fn with_case(ch: char, is_uppercase: bool) -> Self {
        Self { ch, is_uppercase }
    }
    
    /// Get the output character with original case restored
    pub fn to_output_char(&self) -> char {
        if self.is_uppercase {
            self.ch.to_uppercase().next().unwrap_or(self.ch)
        } else {
            self.ch
        }
    }
}

impl From<char> for CharInfo {
    fn from(ch: char) -> Self {
        Self::new(ch)
    }
}

/// Extension trait for Vec<CharInfo> operations
pub trait CharInfoBufferExt {
    /// Convert to normalized (lowercase) string for internal processing
    fn to_normalized_string(&self) -> String;
    
    /// Convert to output string with case restored
    fn to_output_string(&self) -> String;
    
    /// Get just the characters as a vector
    fn to_char_vec(&self) -> Vec<char>;
    
    /// Get the case mask (for backward compatibility)
    fn to_case_mask(&self) -> Vec<bool>;
}

impl CharInfoBufferExt for Vec<CharInfo> {
    fn to_normalized_string(&self) -> String {
        self.iter().map(|c| c.ch).collect()
    }
    
    fn to_output_string(&self) -> String {
        self.iter().map(|c| c.to_output_char()).collect()
    }
    
    fn to_char_vec(&self) -> Vec<char> {
        self.iter().map(|c| c.ch).collect()
    }
    
    fn to_case_mask(&self) -> Vec<bool> {
        self.iter().map(|c| c.is_uppercase).collect()
    }
}

impl CharInfoBufferExt for [CharInfo] {
    fn to_normalized_string(&self) -> String {
        self.iter().map(|c| c.ch).collect()
    }
    
    fn to_output_string(&self) -> String {
        self.iter().map(|c| c.to_output_char()).collect()
    }
    
    fn to_char_vec(&self) -> Vec<char> {
        self.iter().map(|c| c.ch).collect()
    }
    
    fn to_case_mask(&self) -> Vec<bool> {
        self.iter().map(|c| c.is_uppercase).collect()
    }
}

/// Typing Context - Mutable state that flows through the pipeline
///
/// This struct contains all the state needed to process input through
/// the 7-stage pipeline. Each stage can read and modify this context.
///
/// ## Algorithm
///
/// The context maintains three key buffers:
/// - **raw_buffer**: The actual keystrokes typed by the user
/// - **syllable_buffer**: The current Vietnamese syllable being composed
/// - **last_output**: The last text sent to the application
///
/// The pipeline uses these buffers to:
/// 1. Track what the user typed (raw_buffer)
/// 2. Build the Vietnamese syllable (syllable_buffer)
/// 3. Calculate what needs to be changed (diff between last_output and syllable_buffer)
///
/// ## Enhanced State Tracking (Phase 1)
///
/// Additional fields for complex Telex/VNI logic:
/// - **last_char**: Last input character (for pattern matching)
/// - **last_transform_key**: Last key that triggered a transformation
/// - **tone_position**: Explicit tone position override
/// - **flags**: Generic key-value flags for custom rules
#[derive(Debug, Clone)]
pub struct TypingContext {
    /// Character buffer with case tracking (unified raw_buffer + case_mask)
    /// Stores normalized characters with their original case information
    /// Example: "THUOWNGF" stored as [CharInfo('t', true), CharInfo('h', true), ...]
    pub char_buffer: Vec<CharInfo>,

    /// Current syllable being composed
    /// Example: "thường" (after transformations)
    pub syllable_buffer: String,

    /// Last output sent to the application
    /// Used to calculate backspace count for Replace actions
    pub last_output: String,

    /// Cursor position in the syllable buffer
    pub cursor: usize,

    /// Temporary English mode flag
    /// Set to true when undo is detected (double typing modification keys)
    /// Causes subsequent input to pass through as raw Latin
    pub temp_english_mode: bool,

    /// History of transformations (for advanced undo)
    /// Each entry is (raw_buffer, syllable_buffer) at that point
    pub history: Vec<(String, String)>,

    /// Dictionary candidates (from Stage 8: Dictionary Lookup)
    /// Populated when dictionary lookup finds matching entries
    /// Example: For "nguoi" → ["người", "𠊛"]
    pub candidates: Vec<Candidate>,

    /// Whether candidate window is currently showing
    pub showing_candidates: bool,

    /// Selected candidate index (0-based)
    pub selected_candidate: Option<usize>,

    // ========================================
    // Enhanced State Tracking (Phase 1)
    // ========================================
    
    /// Last input character
    /// Used by RuleMatcher::LastChar for pattern matching
    /// Example: For Telex W handling, check if last_char == 'w'
    pub last_char: Option<char>,

    /// Last key that triggered a transformation
    /// Used to track state across transformations
    /// Example: Telex 'w' transforms ư, so last_transform_key = Some('w')
    /// This prevents "ư + w → ư" from transforming again
    pub last_transform_key: Option<char>,

    /// Explicit tone position override
    /// Some rules (like UA in Telex) need to override default tone positioning
    /// Example: In "qua", tone should go on 'u', not 'a'
    pub tone_position: Option<usize>,

    /// Generic flags for custom rules
    /// Allows config functions to store arbitrary state
    /// Example: flags.insert("w_converted", true) for Telex W tracking
    pub flags: HashMap<String, bool>,

    // ========================================
    // Transform History (Phase 1: Undo Support)
    // ========================================
    
    /// History of transformations for undo support
    /// Tracks each transformation to enable Unikey-style undo
    /// Example: "aa" → "â" creates record (input='a', type=Transform)
    pub transform_history: Vec<TransformRecord>,

    // ========================================
    // Legacy fields — kept for context compatibility during transition.
    // These were used by the old Permutation/Reconciliation/Retrofix stages
    // (removed in Phase 4).  ComposeStage handles all of this internally.
    //
    // event-sourcing-completion Phase 8 purity audit: the sibling one-way
    // BOOL flags this block used to carry (`has_pending_marks`,
    // `had_successful_transform`, `used_permutation_result`) were deleted —
    // grep-verified zero production readers anywhere in the crate (only
    // written here at init/clear, never consulted by ComposeStage or any
    // other stage). The two `Option<T>` slots below are out of this phase's
    // scope (bool deletions only) but are equally dead; a future cleanup
    // pass may remove them too.
    // ========================================

    /// Legacy permutation result slot (not populated by ComposeStage).
    pub permutation_result: Option<PermutationResult>,

    /// Incremental result slot (legacy, not used by ComposeStage).
    pub incremental_result: Option<String>,

    // ========================================
    // Learning Support (Stage 5 — future)
    // ========================================

    /// Whether learning is enabled for this session
    pub learning_enabled: bool,

    /// Completed syllables in this session (for learning)
    pub completed_syllables: Vec<String>,

    // ========================================
    // Cross-Word Backspace Support (UniKey Pattern)
    // ========================================

    /// Indices where words start in syllable_buffer
    /// 
    /// When space is typed, the current position is recorded as a word boundary.
    /// This allows:
    /// - Tone processing to only affect current word (chars after last boundary)
    /// - Full buffer preservation for cross-word backspace
    /// 
    /// ## Example
    /// 
    /// Buffer: "tiếp theo"
    /// word_start_indices: [0, 5]  // "tiếp" starts at 0, "theo" starts at 5
    /// current_word(): "theo"      // Only this gets tone processing
    pub word_start_indices: Vec<usize>,
}

/// Result from permutation matching (Stage 6)
///
/// Contains the best candidate found by trying different orderings
/// of marks (transforms + tones) on the base word.
#[derive(Debug, Clone, PartialEq)]
pub struct PermutationResult {
    /// The transformed text (e.g., "tường")
    pub text: String,

    /// Score of this result (higher = better)
    pub score: f32,

    /// The mark ordering that produced this result
    pub mark_order: Vec<char>,
}

/// Record of a single transformation for undo support
///
/// Tracks what happened during a transformation so we can reverse it.
/// Used by Stage 6 (Retrofix) to implement Unikey-style undo.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformRecord {
    /// The input character that triggered this transformation
    /// Example: For "aa" → "â", this is 'a' (the second 'a')
    pub input_char: char,

    /// Buffer state before the transformation
    /// Example: For "aa" → "â", this is "a"
    pub before: String,

    /// Buffer state after the transformation
    /// Example: For "aa" → "â", this is "â"
    pub after: String,

    /// Type of transformation
    pub transform_type: TransformType,
}

/// Type of transformation for undo tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformType {
    /// Character transformation (aa→â, aw→ă, dd→đ, etc.)
    CharTransform,

    /// Tone application (s→sắc, f→huyền, etc.)
    ToneApplication,

    /// Other transformation
    Other,
}

/// Dictionary candidate
///
/// Represents a single candidate from dictionary lookup.
/// Used by Stage 8 (Dictionary Lookup) to provide suggestions.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    /// The candidate text to display (e.g., "𡗶 (trời)")
    /// For Nôm candidates, this includes the Vietnamese meaning in parentheses
    pub text: String,

    /// The actual value to insert when selected (e.g., "𡗶")
    /// For Nôm candidates, this is just the character without the meaning
    /// If None, uses `text` as the value
    pub value: Option<String>,

    /// Candidate type/category
    pub candidate_type: CandidateType,

    /// Relevance score (higher = more relevant)
    /// Used for ranking candidates
    pub score: f32,
}

impl Candidate {
    /// Get the value to insert when this candidate is selected
    pub fn get_value(&self) -> &str {
        self.value.as_deref().unwrap_or(&self.text)
    }
}

/// Type of dictionary candidate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateType {
    /// Standard Vietnamese word
    Vietnamese,

    /// Chữ Nôm character
    Nom,

    /// English word (passthrough)
    English,

    /// Other/Unknown
    Other,
}

impl TypingContext {
    /// Create a new empty context
    ///
    /// ## Algorithm
    ///
    /// Initializes all buffers to empty strings and sets flags to default values.
    pub fn new() -> Self {
        Self {
            char_buffer: Vec::new(),
            syllable_buffer: String::new(),
            last_output: String::new(),
            cursor: 0,
            temp_english_mode: false,
            history: Vec::new(),
            candidates: Vec::new(),
            showing_candidates: false,
            selected_candidate: None,
            // Enhanced state tracking
            last_char: None,
            last_transform_key: None,
            tone_position: None,
            flags: HashMap::new(),
            transform_history: Vec::new(),
            // Legacy permutation fields (not used by ComposeStage)
            permutation_result: None,
            incremental_result: None,
            // Learning (Stage 5, future)
            learning_enabled: false,
            completed_syllables: Vec::new(),
            // Cross-word backspace support
            word_start_indices: vec![0],  // First word always starts at 0
        }
    }

    /// Clear all buffers and reset state
    ///
    /// ## Algorithm
    ///
    /// Resets the context to initial state, clearing all buffers and history.
    /// This is called when the user moves to a new word or context.
    pub fn clear(&mut self) {
        self.char_buffer.clear();
        self.syllable_buffer.clear();
        self.last_output.clear();
        self.cursor = 0;
        self.temp_english_mode = false;
        self.history.clear();
        self.candidates.clear();
        self.showing_candidates = false;
        self.selected_candidate = None;
        // Clear enhanced state
        self.last_char = None;
        self.last_transform_key = None;
        self.tone_position = None;
        self.flags.clear();
        self.transform_history.clear();
        // Clear legacy permutation state
        self.permutation_result = None;
        self.incremental_result = None;
        // Cross-word backspace support
        self.word_start_indices = vec![0];  // Reset with first word at 0
        // Note: Don't clear learning_enabled or completed_syllables (session-level)
    }

    /// Save current state to history
    ///
    /// ## Algorithm
    ///
    /// Pushes a snapshot of (raw_buffer, syllable_buffer) to the history stack.
    /// This allows for multi-level undo in the future.
    pub fn save_to_history(&mut self) {
        self.history.push((
            self.raw_buffer(),
            self.syllable_buffer.clone(),
        ));
    }

    /// Check if buffers are empty
    pub fn is_empty(&self) -> bool {
        self.char_buffer.is_empty() && self.syllable_buffer.is_empty()
    }

    /// Get the length of the syllable buffer
    pub fn len(&self) -> usize {
        self.syllable_buffer.chars().count()
    }

    /// Append a character to the char buffer
    ///
    /// ## Algorithm
    ///
    /// Creates a CharInfo from the input and adds it to char_buffer.
    /// The character is normalized (lowercased) and case info is preserved.
    pub fn push_raw(&mut self, ch: char) {
        self.char_buffer.push(CharInfo::new(ch));
    }

    /// Append a CharInfo directly to the buffer
    pub fn push_char_info(&mut self, info: CharInfo) {
        self.char_buffer.push(info);
    }

    // ========================================
    // Backward Compatibility Getters
    // ========================================

    /// Get raw buffer as string (normalized, for backward compatibility)
    /// 
    /// Returns the characters only (without case info) as a string.
    pub fn raw_buffer(&self) -> String {
        self.char_buffer.to_normalized_string()
    }

    /// Get case mask (for backward compatibility)
    pub fn case_mask(&self) -> Vec<bool> {
        self.char_buffer.to_case_mask()
    }

    /// Set char_buffer from a string (for testing/backward compatibility)
    /// 
    /// Converts each character in the string to CharInfo and replaces char_buffer.
    /// All characters are assumed lowercase (is_uppercase = false).
    pub fn set_raw_buffer(&mut self, s: &str) {
        self.char_buffer = s.chars().map(CharInfo::new).collect();
    }

    /// Set case mask for existing char_buffer (for testing)
    /// 
    /// Updates the is_uppercase flag for each character in char_buffer.
    /// The case_mask length should match char_buffer length.
    pub fn set_case_mask(&mut self, case_mask: Vec<bool>) {
        for (i, is_upper) in case_mask.into_iter().enumerate() {
            if i < self.char_buffer.len() {
                self.char_buffer[i].is_uppercase = is_upper;
            }
        }
    }

    /// Set the syllable buffer to a new value
    ///
    /// ## Algorithm
    ///
    /// Replaces the syllable_buffer with a new transformed value.
    /// This is used by transformation stages to apply Vietnamese transformations.
    pub fn set_syllable(&mut self, syllable: String) {
        self.syllable_buffer = syllable;
    }

    // ========================================
    // Cross-Word Backspace Support
    // ========================================

    /// Get the start index of the current word in syllable_buffer
    /// 
    /// Returns the byte offset where the current word starts.
    /// If no word boundaries exist, returns 0.
    pub fn current_word_start(&self) -> usize {
        self.word_start_indices.last().copied().unwrap_or(0)
    }

    /// Get the current word (substring from last word boundary to end)
    /// 
    /// This is the portion of syllable_buffer that tone/transform stages
    /// should operate on. Characters before this are from previous words.
    /// 
    /// ## Example
    /// 
    /// Buffer: "tiếp theo"
    /// word_start_indices: [0, 5]
    /// current_word(): "theo"
    pub fn current_word(&self) -> &str {
        let start = self.current_word_start();
        if start < self.syllable_buffer.len() {
            &self.syllable_buffer[start..]
        } else {
            ""
        }
    }

    /// Mark current position as a word boundary
    /// 
    /// Called when a space or soft separator is typed. Records the
    /// current syllable_buffer length as the start of the next word.
    pub fn mark_word_boundary(&mut self) {
        self.word_start_indices.push(self.syllable_buffer.len());
    }

    /// Remove the last word boundary (on backspace that removes separator)
    /// 
    /// Called when backspace removes a space/separator, merging the
    /// current word with the previous one.
    pub fn pop_word_boundary(&mut self) {
        // Always keep at least one entry (index 0)
        if self.word_start_indices.len() > 1 {
            self.word_start_indices.pop();
        }
    }

    /// Check if we have previous words in buffer (for cross-word backspace)
    pub fn has_previous_word(&self) -> bool {
        self.word_start_indices.len() > 1
    }

    /// Get char count offset for current word (for tone position calculation)
    /// 
    /// Since word_start_indices stores byte offsets but tone positioning uses
    /// char indices, this method converts byte offset to char count.
    pub fn current_word_char_offset(&self) -> usize {
        let byte_start = self.current_word_start();
        // Count chars from start to byte_start position
        self.syllable_buffer[..byte_start].chars().count()
    }

    // ========================================
    // Candidate Selection Methods
    // ========================================

    /// Check if there are candidates available
    pub fn has_candidates(&self) -> bool {
        !self.candidates.is_empty()
    }

    /// Get the number of candidates
    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    /// Select next candidate (cycle forward)
    ///
    /// ## Algorithm
    ///
    /// Moves selection to next candidate, wrapping around to 0 if at end.
    /// If no candidate is selected, selects the first one.
    pub fn select_next_candidate(&mut self) {
        if self.candidates.is_empty() {
            return;
        }

        self.selected_candidate = Some(match self.selected_candidate {
            Some(idx) => (idx + 1) % self.candidates.len(),
            None => 0,
        });
    }

    /// Select previous candidate (cycle backward)
    ///
    /// ## Algorithm
    ///
    /// Moves selection to previous candidate, wrapping around to end if at start.
    /// If no candidate is selected, selects the last one.
    pub fn select_previous_candidate(&mut self) {
        if self.candidates.is_empty() {
            return;
        }

        self.selected_candidate = Some(match self.selected_candidate {
            Some(idx) if idx > 0 => idx - 1,
            Some(_) => self.candidates.len() - 1,
            None => self.candidates.len() - 1,
        });
    }

    /// Select candidate by index
    ///
    /// ## Algorithm
    ///
    /// Sets selected_candidate to the given index if valid.
    /// Returns true if successful, false if index out of bounds.
    pub fn select_candidate(&mut self, index: usize) -> bool {
        if index < self.candidates.len() {
            self.selected_candidate = Some(index);
            true
        } else {
            false
        }
    }

    /// Get currently selected candidate
    ///
    /// ## Returns
    ///
    /// The selected Candidate if one is selected, None otherwise.
    pub fn get_selected_candidate(&self) -> Option<&Candidate> {
        self.selected_candidate
            .and_then(|idx| self.candidates.get(idx))
    }

    /// Get candidate by index
    pub fn get_candidate(&self, index: usize) -> Option<&Candidate> {
        self.candidates.get(index)
    }

    /// Accept currently selected candidate
    ///
    /// ## Algorithm
    ///
    /// 1. Get selected candidate text
    /// 2. Update syllable_buffer with candidate text
    /// 3. Clear candidates and selection
    /// 4. Return the accepted text
    ///
    /// ## Returns
    ///
    /// The accepted candidate text, or None if no candidate selected.
    pub fn accept_selected_candidate(&mut self) -> Option<String> {
        if let Some(candidate) = self.get_selected_candidate() {
            let text = candidate.text.clone();
            
            // Update syllable buffer with accepted candidate
            self.syllable_buffer = text.clone();
            
            // Clear candidates
            self.candidates.clear();
            self.showing_candidates = false;
            self.selected_candidate = None;
            
            Some(text)
        } else {
            None
        }
    }

    /// Dismiss candidate window
    ///
    /// ## Algorithm
    ///
    /// Clears all candidates and resets selection state.
    pub fn dismiss_candidates(&mut self) {
        self.candidates.clear();
        self.showing_candidates = false;
        self.selected_candidate = None;
    }

    /// Update last_output to match current syllable
    ///
    /// ## Algorithm
    ///
    /// Called after generating output actions to sync last_output with
    /// what was actually sent to the application.
    pub fn commit_output(&mut self) {
        // clone_from reuses last_output's existing allocation (hot path:
        // called once per keystroke by OutputStage).
        self.last_output.clone_from(&self.syllable_buffer);
    }

    // ========================================
    // Enhanced State Tracking Methods
    // ========================================

    /// Set a custom flag
    ///
    /// ## Purpose:
    /// Allows custom rules to store arbitrary boolean state.
    /// Used by ContextRule actions to track complex conditions.
    ///
    /// ## Example:
    /// ```rust,ignore
    /// ctx.set_flag("w_converted", true);
    /// ```
    pub fn set_flag(&mut self, name: &str, value: bool) {
        self.flags.insert(name.to_string(), value);
    }

    /// Get a custom flag value
    ///
    /// ## Returns:
    /// The flag value, or false if not set.
    pub fn get_flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    /// Clear all custom flags
    pub fn clear_flags(&mut self) {
        self.flags.clear();
    }
}

impl Default for TypingContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_context() {
        let ctx = TypingContext::new();
        assert!(ctx.is_empty());
        assert_eq!(ctx.len(), 0);
        assert!(!ctx.temp_english_mode);
    }

    #[test]
    fn test_push_raw() {
        let mut ctx = TypingContext::new();
        ctx.push_raw('a');
        assert_eq!(ctx.raw_buffer(), "a");
        assert!(ctx.syllable_buffer.is_empty());
    }

    #[test]
    fn test_set_syllable() {
        let mut ctx = TypingContext::new();
        ctx.set_syllable("â".to_string());
        assert_eq!(ctx.syllable_buffer, "â");
        assert_eq!(ctx.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut ctx = TypingContext::new();
        ctx.push_raw('a');
        ctx.set_syllable("â".to_string());
        ctx.temp_english_mode = true;
        
        ctx.clear();
        
        assert!(ctx.is_empty());
        assert!(!ctx.temp_english_mode);
        assert_eq!(ctx.history.len(), 0);
    }

    #[test]
    fn test_history() {
        let mut ctx = TypingContext::new();
        ctx.push_raw('a');
        ctx.set_syllable("a".to_string());
        ctx.save_to_history();
        
        ctx.push_raw('a');
        ctx.set_syllable("â".to_string());
        ctx.save_to_history();
        
        assert_eq!(ctx.history.len(), 2);
        assert_eq!(ctx.history[0], ("a".to_string(), "a".to_string()));
        assert_eq!(ctx.history[1], ("aa".to_string(), "â".to_string()));
    }

    #[test]
    fn test_commit_output() {
        let mut ctx = TypingContext::new();
        ctx.set_syllable("thường".to_string());
        ctx.commit_output();
        
        assert_eq!(ctx.last_output, "thường");
    }

    // Candidate Selection Tests

    #[test]
    fn test_has_candidates() {
        let mut ctx = TypingContext::new();
        assert!(!ctx.has_candidates());

        ctx.candidates.push(Candidate {
            text: "người".to_string(),
            value: None,
            candidate_type: CandidateType::Vietnamese,
            score: 1.0,
        });
        assert!(ctx.has_candidates());
    }

    #[test]
    fn test_candidate_count() {
        let mut ctx = TypingContext::new();
        assert_eq!(ctx.candidate_count(), 0);

        ctx.candidates.push(Candidate {
            text: "người".to_string(),
            value: None,
            candidate_type: CandidateType::Vietnamese,
            score: 1.0,
        });
        ctx.candidates.push(Candidate {
            text: "𠊛".to_string(),
            value: None,
            candidate_type: CandidateType::Nom,
            score: 0.5,
        });
        assert_eq!(ctx.candidate_count(), 2);
    }

    #[test]
    fn test_select_next_candidate() {
        let mut ctx = TypingContext::new();
        
        // Add 3 candidates
        for i in 0..3 {
            ctx.candidates.push(Candidate {
                text: format!("candidate{}", i),
                value: None,
                candidate_type: CandidateType::Vietnamese,
                score: 1.0,
            });
        }

        // Initially no selection
        assert_eq!(ctx.selected_candidate, None);

        // First next -> select 0
        ctx.select_next_candidate();
        assert_eq!(ctx.selected_candidate, Some(0));

        // Second next -> select 1
        ctx.select_next_candidate();
        assert_eq!(ctx.selected_candidate, Some(1));

        // Third next -> select 2
        ctx.select_next_candidate();
        assert_eq!(ctx.selected_candidate, Some(2));

        // Fourth next -> wrap to 0
        ctx.select_next_candidate();
        assert_eq!(ctx.selected_candidate, Some(0));
    }

    #[test]
    fn test_select_previous_candidate() {
        let mut ctx = TypingContext::new();
        
        // Add 3 candidates
        for i in 0..3 {
            ctx.candidates.push(Candidate {
                text: format!("candidate{}", i),
                value: None,
                candidate_type: CandidateType::Vietnamese,
                score: 1.0,
            });
        }

        // Initially no selection
        assert_eq!(ctx.selected_candidate, None);

        // First previous -> select last (2)
        ctx.select_previous_candidate();
        assert_eq!(ctx.selected_candidate, Some(2));

        // Second previous -> select 1
        ctx.select_previous_candidate();
        assert_eq!(ctx.selected_candidate, Some(1));

        // Third previous -> select 0
        ctx.select_previous_candidate();
        assert_eq!(ctx.selected_candidate, Some(0));

        // Fourth previous -> wrap to last (2)
        ctx.select_previous_candidate();
        assert_eq!(ctx.selected_candidate, Some(2));
    }

    #[test]
    fn test_select_candidate_by_index() {
        let mut ctx = TypingContext::new();
        
        // Add 3 candidates
        for i in 0..3 {
            ctx.candidates.push(Candidate {
                text: format!("candidate{}", i),
                value: None,
                candidate_type: CandidateType::Vietnamese,
                score: 1.0,
            });
        }

        // Select valid index
        assert!(ctx.select_candidate(1));
        assert_eq!(ctx.selected_candidate, Some(1));

        // Select another valid index
        assert!(ctx.select_candidate(2));
        assert_eq!(ctx.selected_candidate, Some(2));

        // Select invalid index
        assert!(!ctx.select_candidate(10));
        assert_eq!(ctx.selected_candidate, Some(2)); // Unchanged
    }

    #[test]
    fn test_get_selected_candidate() {
        let mut ctx = TypingContext::new();
        
        ctx.candidates.push(Candidate {
            text: "người".to_string(),
            value: None,
            candidate_type: CandidateType::Vietnamese,
            score: 1.0,
        });
        ctx.candidates.push(Candidate {
            text: "𠊛".to_string(),
            value: None,
            candidate_type: CandidateType::Nom,
            score: 0.5,
        });

        // No selection
        assert!(ctx.get_selected_candidate().is_none());

        // Select first
        ctx.select_candidate(0);
        let selected = ctx.get_selected_candidate().unwrap();
        assert_eq!(selected.text, "người");

        // Select second
        ctx.select_candidate(1);
        let selected = ctx.get_selected_candidate().unwrap();
        assert_eq!(selected.text, "𠊛");
    }

    #[test]
    fn test_get_candidate() {
        let mut ctx = TypingContext::new();
        
        ctx.candidates.push(Candidate {
            text: "người".to_string(),
            value: None,
            candidate_type: CandidateType::Vietnamese,
            score: 1.0,
        });

        assert!(ctx.get_candidate(0).is_some());
        assert_eq!(ctx.get_candidate(0).unwrap().text, "người");
        assert!(ctx.get_candidate(1).is_none());
    }

    #[test]
    fn test_accept_selected_candidate() {
        let mut ctx = TypingContext::new();
        ctx.syllable_buffer = "nguoi".to_string();
        
        ctx.candidates.push(Candidate {
            text: "người".to_string(),
            value: None,
            candidate_type: CandidateType::Vietnamese,
            score: 1.0,
        });
        ctx.candidates.push(Candidate {
            text: "𠊛".to_string(),
            value: None,
            candidate_type: CandidateType::Nom,
            score: 0.5,
        });
        ctx.showing_candidates = true;

        // No selection -> returns None
        assert!(ctx.accept_selected_candidate().is_none());

        // Select and accept
        ctx.select_candidate(1);
        let accepted = ctx.accept_selected_candidate().unwrap();
        
        assert_eq!(accepted, "𠊛");
        assert_eq!(ctx.syllable_buffer, "𠊛");
        assert!(!ctx.has_candidates());
        assert!(!ctx.showing_candidates);
        assert_eq!(ctx.selected_candidate, None);
    }

    #[test]
    fn test_dismiss_candidates() {
        let mut ctx = TypingContext::new();
        
        ctx.candidates.push(Candidate {
            text: "người".to_string(),
            value: None,
            candidate_type: CandidateType::Vietnamese,
            score: 1.0,
        });
        ctx.showing_candidates = true;
        ctx.selected_candidate = Some(0);

        ctx.dismiss_candidates();

        assert!(!ctx.has_candidates());
        assert!(!ctx.showing_candidates);
        assert_eq!(ctx.selected_candidate, None);
    }

    #[test]
    fn test_candidate_selection_empty_list() {
        let mut ctx = TypingContext::new();

        // Operations on empty list should not crash
        ctx.select_next_candidate();
        assert_eq!(ctx.selected_candidate, None);

        ctx.select_previous_candidate();
        assert_eq!(ctx.selected_candidate, None);

        assert!(!ctx.select_candidate(0));
        assert!(ctx.get_selected_candidate().is_none());
        assert!(ctx.accept_selected_candidate().is_none());
    }
}
