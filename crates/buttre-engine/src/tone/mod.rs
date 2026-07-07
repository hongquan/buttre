//! Tone module: single source of truth for Vietnamese tone character mappings and placement.
//!
//! ## Responsibilities
//!
//! - **`tables`**: `apply` / `strip` — char-level tone application and stripping.
//! - **`placement`**: `place` — which vowel in a nucleus receives the tone mark.
//!
//! ## What this module does NOT own
//!
//! - Key→ToneMark mapping (lives in `PipelineConfig::tone_map` and in each stage).
//! - Syllable parsing / final-consonant detection (caller responsibility).

mod placement;
mod tables;

pub use placement::place;
pub use tables::{apply, strip};
