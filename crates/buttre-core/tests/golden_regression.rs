//! Golden regression tests — read .snap files and verify the current engine
//! produces identical output for every case.
//!
//! ## Running
//!
//!     cargo test -p buttre-core golden_regression
//!
//! ## Generating / regenerating snapshots
//!
//!     cargo run -p buttre-core --example gen_golden
//!
//! Snapshots live in `tests/golden/{telex,vni,nom}.snap`.
//! Format: `<keys>\t<expected_output>\t<TAG>` one per line.

mod golden;

use buttre_core::keyboard::{nom, telex, vni};
use buttre_engine::pipeline::PipelineConfig;
use golden::type_sequence;

use std::path::{Path, PathBuf};

// ── snapshot loader ───────────────────────────────────────────────────────────

struct SnapCase {
    keys: String,
    expected: String,
    tag: String,
}

fn load_snap(path: &Path) -> Vec<SnapCase> {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Cannot read snap file {}: {e}", path.display()));

    content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.splitn(3, '\t').collect();
            assert!(
                parts.len() == 3,
                "Malformed snap line (expected 3 tab-separated fields): {:?}",
                line
            );
            SnapCase {
                keys: parts[0].to_string(),
                expected: parts[1].to_string(),
                tag: parts[2].to_string(),
            }
        })
        .collect()
}

fn snap_path(method: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join(format!("{method}.snap"))
}

// ── generic runner ────────────────────────────────────────────────────────────

fn run_regression(method: &str, config_fn: fn() -> PipelineConfig) {
    let path = snap_path(method);
    if !path.exists() {
        panic!(
            "{method}.snap not found at {} — run: cargo run -p buttre-core --example gen_golden",
            path.display()
        );
    }
    let cases = load_snap(&path);
    assert!(
        !cases.is_empty(),
        "{method}.snap is empty — run gen_golden first"
    );

    let mut failures = 0usize;
    let mut failure_msgs: Vec<String> = Vec::new();

    for case in &cases {
        let actual = type_sequence(config_fn(), &case.keys);
        if actual != case.expected {
            failures += 1;
            failure_msgs.push(format!(
                "  keys: {} expected '{}' got '{}' [{}]",
                case.keys, case.expected, actual, case.tag
            ));
            // Collect all failures before panicking for better diagnostics.
            if failures >= 50 {
                failure_msgs.push("  … (more failures truncated)".to_string());
                break;
            }
        }
    }

    assert!(
        failures == 0,
        "\n{method} regression: {failures} failure(s):\n{}",
        failure_msgs.join("\n")
    );

    println!("[golden_regression] {method}: {} cases OK", cases.len());
}

// ── per-method test functions ─────────────────────────────────────────────────

#[test]
fn test_golden_telex() {
    run_regression("telex", telex::build_config);
}

#[test]
fn test_golden_vni() {
    run_regression("vni", vni::build_config);
}

/// Nôm regression — skipped automatically if nom.snap does not exist
/// (which happens when buttre_nom.db was absent during gen_golden).
#[test]
fn test_golden_nom() {
    let path = snap_path("nom");
    if !path.exists() {
        println!("[golden_regression] nom: skipped (nom.snap not present)");
        return;
    }
    run_regression("nom", nom::build_config);
}
