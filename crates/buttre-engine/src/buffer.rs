//! Input buffer management for character handling
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/buffer_tests.rs`.
//!
//! This module provides the `InputBuffer` type for managing typed characters
//! with automatic overflow handling and capacity management.
//!
//! ## Key Features
//!
//! - **Fixed capacity**: 40 characters maximum (compatible with UniKey)
//! - **Automatic overflow**: Keeps last 20 characters when full
//! - **Case tracking**: Remembers original case of each character
//! - **Flags**: Tracks conversion state and escape sequences

/// Maximum buffer size (same as UniKey)
const BUFFER_SIZE: usize = 40;

/// Number of characters to keep when buffer is full
const KEYS_MAINTAIN: usize = 20;

/// Input buffer for managing typed characters
#[derive(Debug, Clone)]
pub struct InputBuffer {
    /// Characters in the buffer
    chars: Vec<char>,

    /// Lowercase flags for each character
    lowercase_flags: Vec<bool>,

    /// Last 'w' was converted to 'ư' (Telex specific)
    last_w_converted: bool,

    /// Last character was an escape character (VIQR specific)
    last_is_escape: bool,
}

impl InputBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            chars: Vec::with_capacity(BUFFER_SIZE),
            lowercase_flags: Vec::with_capacity(BUFFER_SIZE),
            last_w_converted: false,
            last_is_escape: false,
        }
    }

    /// Push a character onto the buffer
    pub fn push(&mut self, ch: char, is_lowercase: bool) {
        if self.chars.len() >= BUFFER_SIZE {
            self.throw_buffer();
        }
        self.chars.push(ch);
        self.lowercase_flags.push(is_lowercase);
    }

    /// Pop the last character from the buffer
    pub fn pop(&mut self) -> Option<(char, bool)> {
        let ch = self.chars.pop()?;
        let flag = self.lowercase_flags.pop()?;
        Some((ch, flag))
    }

    /// Get the last character without removing it
    pub fn last(&self) -> Option<&char> {
        self.chars.last()
    }

    /// Get character at specific position
    pub fn get(&self, index: usize) -> Option<&char> {
        self.chars.get(index)
    }

    /// Set character at specific position
    pub fn set(&mut self, index: usize, ch: char) {
        if index < self.chars.len() {
            self.chars[index] = ch;
        }
    }

    /// Get the number of characters in the buffer
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// Clear the entire buffer
    pub fn clear(&mut self) {
        self.chars.clear();
        self.lowercase_flags.clear();
        self.last_w_converted = false;
        self.last_is_escape = false;
    }

    /// Get iterator over characters from a specific position
    pub fn chars_from(&self, start: usize) -> impl Iterator<Item = char> + '_ {
        self.chars[start..].iter().copied()
    }

    /// Get all characters as a string
    pub fn to_string(&self) -> String {
        self.chars.iter().collect()
    }

    /// Throw buffer - keep only last KEYS_MAINTAIN characters
    /// Called when buffer is full
    fn throw_buffer(&mut self) {
        if self.chars.len() > KEYS_MAINTAIN {
            let drain_count = self.chars.len() - KEYS_MAINTAIN;
            self.chars.drain(0..drain_count);
            self.lowercase_flags.drain(0..drain_count);
        }
    }

    /// Get the last_w_converted flag
    ///
    /// Returns whether the last 'w' character was converted to a Vietnamese character.
    pub fn last_w_converted(&self) -> bool {
        self.last_w_converted
    }

    /// Set the last_w_converted flag
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the last 'w' was converted
    pub fn set_last_w_converted(&mut self, value: bool) {
        self.last_w_converted = value;
    }

    /// Get the last_is_escape flag
    ///
    /// Returns whether the last character was an escape sequence.
    pub fn last_is_escape(&self) -> bool {
        self.last_is_escape
    }

    /// Set the last_is_escape flag
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the last character was an escape
    pub fn set_last_is_escape(&mut self, value: bool) {
        self.last_is_escape = value;
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

