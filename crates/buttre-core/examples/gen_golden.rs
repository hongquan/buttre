//! Snapshot generator for the golden regression harness.
//!
//! ## Usage
//!
//!     cargo run -p buttre-core --example gen_golden
//!
//! Writes (or overwrites):
//! - `crates/buttre-core/tests/golden/telex.snap`
//! - `crates/buttre-core/tests/golden/vni.snap`
//! - `crates/buttre-core/tests/golden/nom.snap`  (only if buttre_nom.db found)
//!
//! ## Format
//!
//! Each snapshot is a plain-text file, one case per line:
//!
//!     <keys>\t<expected_output>\t<TAG>
//!
//! ## Determinism
//!
//! No randomness or time-based values are used.
//!
//! ## Single source of truth
//!
//! The corpus (syllable list, decompose tables, Tag enum) lives entirely in
//! `tests/golden/corpus_data/` and is included here via `#[path]`.  gen_golden
//! and golden_regression.rs both consume that exact module — there is no
//! second copy of the syllable list anywhere.

use std::fs;
use std::path::{Path, PathBuf};

use buttre_core::keyboard::{nom, telex, vni};
use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};
use buttre_engine::types::Action;

// ── Pull corpus from tests/golden/corpus_data/ ────────────────────────────────
// This is the single source of truth; golden_regression.rs imports the same
// module via `mod golden; golden::corpus_data::…`.

#[path = "../tests/golden/corpus_data/mod.rs"]
mod corpus_data;

use corpus_data::{nom_corpus, telex_corpus, vni_corpus, Tag};

// ── replay / type_sequence ────────────────────────────────────────────────────

/// Simulate the host-application text buffer by replaying engine actions.
///
/// `backspace_count` is clamped to `buf.len()` for panic-safety; callers that
/// need to detect over-backspace must check before calling.
fn replay(actions: &[Action]) -> String {
    let mut buf: Vec<char> = Vec::new();
    for action in actions {
        match action {
            Action::Commit(s) => buf.extend(s.chars()),
            Action::Replace {
                backspace_count,
                text,
            } => {
                let remove = (*backspace_count).min(buf.len());
                buf.truncate(buf.len() - remove);
                buf.extend(text.chars());
            }
            Action::ConfirmComposition(s) => buf.extend(s.chars()),
            Action::UpdateComposition { .. }
            | Action::DoNothing
            | Action::ShowCandidates { .. }
            | Action::HideCandidates => {}
        }
    }
    buf.into_iter().collect()
}

/// Drive a fresh `PipelineExecutor` with `keys` and return the final visible
/// text, while warning to stderr for any over-backspace detected during replay.
///
/// Over-backspace (engine emits `backspace_count > current buffer length`) is
/// a potential engine defect.  We clamp it here so generation does not panic,
/// but print a warning so the defect is visible without failing the generator.
fn type_sequence(config: PipelineConfig, keys: &str) -> String {
    let mut executor = PipelineExecutor::new(config);
    let mut all: Vec<Action> = Vec::new();
    for ch in keys.chars() {
        all.extend(executor.process(ch));
    }

    // Detect over-backspace before collapsing into final string.
    let mut buf_len: usize = 0;
    for action in &all {
        match action {
            Action::Commit(s) => buf_len += s.chars().count(),
            Action::Replace {
                backspace_count,
                text,
            } => {
                if *backspace_count > buf_len {
                    eprintln!(
                        "[gen_golden] WARN over-backspace: keys={:?} \
                         backspace_count={} buf_len={} text={:?}",
                        keys, backspace_count, buf_len, text
                    );
                }
                buf_len = buf_len.saturating_sub(*backspace_count);
                buf_len += text.chars().count();
            }
            Action::ConfirmComposition(s) => buf_len += s.chars().count(),
            _ => {}
        }
    }

    replay(&all)
}

// ── snapshot writer ───────────────────────────────────────────────────────────

fn write_snap(path: &Path, corpus: &[(String, Tag)], config_fn: fn() -> PipelineConfig) {
    let mut lines: Vec<String> = Vec::with_capacity(corpus.len());
    for (keys, tag) in corpus {
        let output = type_sequence(config_fn(), keys);
        lines.push(format!("{}\t{}\t{}", keys, output, tag.as_str()));
    }
    let content = lines.join("\n") + "\n";
    fs::write(path, &content).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    println!(
        "[gen_golden] wrote {} — {} cases",
        path.display(),
        lines.len()
    );
}

fn snap_path(method: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join(format!("{method}.snap"))
}

fn main() {
    let telex_corpus = telex_corpus();
    println!("[gen_golden] telex corpus: {} cases", telex_corpus.len());
    write_snap(&snap_path("telex"), &telex_corpus, telex::build_config);

    let vni_corpus = vni_corpus();
    println!("[gen_golden] vni corpus: {} cases", vni_corpus.len());
    write_snap(&snap_path("vni"), &vni_corpus, vni::build_config);

    let nom_db = buttre_core::vietnamese::get_nom_db_path();
    if nom_db.is_some() {
        let nom_corpus = nom_corpus();
        println!("[gen_golden] nom corpus: {} cases", nom_corpus.len());
        write_snap(&snap_path("nom"), &nom_corpus, nom::build_config);
    } else {
        println!("[gen_golden] skipping nom.snap — buttre_nom.db not found");
    }

    println!("[gen_golden] done.");
}
