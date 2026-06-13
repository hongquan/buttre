//! Nôm (Chữ Nôm) dictionary support
//!
//! This module provides SQLite-based dictionary lookup for Chữ Nôm,
//! the traditional Vietnamese writing system using modified Chinese characters.
//!
//! ## Features
//!
//! - SQLite-based storage for efficient lookups
//! - Thread-safe access with mutex protection
//! - Integration with the pipeline's dictionary system

use crate::pipeline::{Candidate, CandidateType, dictionary::DictionaryProvider};
use rusqlite::{Connection, OpenFlags};
use std::path::PathBuf;
use std::sync::Mutex;
use log::{info, error, warn};

/// SQLite-based Nôm dictionary implementation
pub struct NomDictionary {
    conn: Mutex<Connection>,
}

impl NomDictionary {
    /// Open a Nôm dictionary database file
    pub fn open(path: PathBuf) -> anyhow::Result<Self> {
        info!("Opening Nôm dictionary at {:?}", path);
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

impl DictionaryProvider for NomDictionary {
    fn lookup(&self, keyword: &str) -> Vec<Candidate> {
        let conn = match self.conn.lock() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to lock dictionary connection: {}", e);
                return vec![];
            }
        };

        // Use FTS5 for fast search
        // Support two modes:
        // 1. Single keyword: "thien*" - prefix search for candidates starting with "thien"
        // 2. Multi-keyword: "thien* AND thuong*" - AND search for all keywords
        //    Example: "thien thuong" finds characters with meaning containing both words
        
        // Build FTS5 query
        let fts_query = if keyword.contains(' ') {
            // Multi-keyword search: split by space and join with AND
            // "thien thuong" → "thien* AND thuong*"
            keyword.split_whitespace()
                .map(|word| format!("{}*", word))
                .collect::<Vec<_>>()
                .join(" AND ")
        } else {
            // Single keyword: simple prefix search
            format!("{}*", keyword)
        };
        
        // Query FTS5 table and join with main table to get frequency
        // ORDER BY freq DESC to show most common characters first
        // Schema: nom_data (id, char, keywords, meaning, freq, metadata)
        let mut stmt = match conn.prepare(
            "SELECT nom_data.char, nom_data.freq, nom_data.meaning
             FROM nom_fts 
             INNER JOIN nom_data ON nom_fts.rowid = nom_data.id
             WHERE nom_fts.keywords MATCH ?1
             ORDER BY nom_data.freq DESC 
             LIMIT 20"
        ) {
            Ok(s) => s,
            Err(e) => {
                warn!("FTS5 query prepare failed: {}", e);
                return vec![];
            }
        };

        let candidate_iter = match stmt.query_map([fts_query], |row| {
            let char: String = row.get(0)?;
            let freq: i64 = row.get(1).unwrap_or(0);
            let meaning: String = row.get(2).unwrap_or_default();
            
            // Display format: "𡗶 (trời)" if meaning exists, otherwise just the character
            let display_text = if !meaning.is_empty() {
                format!("{} ({})", char, meaning)
            } else {
                char.clone()
            };
            
            // Value is always just the Nôm character (without meaning in parentheses)
            let value = if !meaning.is_empty() {
                Some(char)
            } else {
                None // If no meaning, text == value, so no need to duplicate
            };
            
            Ok(Candidate {
                text: display_text,
                value,
                candidate_type: CandidateType::Nom,
                score: (freq as f32) / 1000.0, // Normalize to 0-1000 range
            })
        }) {
            Ok(iter) => iter,
            Err(e) => {
                warn!("FTS5 query execution failed: {}", e);
                return vec![];
            }
        };

        candidate_iter
            .filter_map(Result::ok)
            .collect()
    }

    fn contains(&self, keyword: &str) -> bool {
        let conn = match self.conn.lock() {
            Ok(c) => c,
            Err(_) => return false,
        };

        // Use FTS5 prefix search to check if any keyword starts with the input
        let fts_query = format!("{}*", keyword);
        let mut stmt = match conn.prepare(
            "SELECT 1 FROM nom_fts WHERE keywords MATCH ?1 LIMIT 1"
        ) {
            Ok(s) => s,
            Err(_) => return false,
        };

        stmt.exists([fts_query]).unwrap_or(false)
    }

    fn count(&self) -> usize {
        // Optional
        0
    }
}
