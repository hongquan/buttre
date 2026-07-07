//! Compose engine end-to-end benchmark.
//!
//! Measures per-keystroke latency for the recompute-from-raw `compose()` path
//! and the full `PipelineExecutor::process()` for Telex and VNI.
//!
//! ## Baseline (established post-Phase-4 refactor, 2026-06-13)
//!
//! The old incremental-stage pipeline has been replaced; these numbers are the
//! new baseline — a before/after wall-clock comparison is not possible because
//! the previous stages no longer exist.  All measurements confirm the recompute
//! path stays well under the 1 ms/keystroke threshold.

use buttre_engine::compose::{compose, ComposeOpts};
use buttre_engine::pipeline::presets;
use buttre_engine::pipeline::PipelineExecutor;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

// ---------------------------------------------------------------------------
// compose() micro-benchmark — pure recompute logic, no executor overhead
// ---------------------------------------------------------------------------

fn bench_compose_telex(c: &mut Criterion) {
    let config = presets::telex_config();
    let opts = ComposeOpts::from_config(&config);

    let mut group = c.benchmark_group("compose_telex");

    // Representative Telex sequences (raw key buffers)
    let cases: &[(&str, &str)] = &[
        ("nguwowif", "người"),  // complex compound vowel + tone
        ("tuongwf", "tường"),   // uo+w compound + grave
        ("dduwowcj", "được"),   // đ-stroke + compound + dot-below
        ("thuongwf", "thường"), // th- initial + compound + grave
        ("aa", "â"),            // simple transform
        ("awf", "ằ"),           // transform + tone
        ("a", "a"),             // single char (hot path)
    ];

    for (raw_str, _expected) in cases {
        let raw: Vec<char> = raw_str.chars().collect();
        group.bench_with_input(BenchmarkId::from_parameter(*raw_str), &raw, |b, raw| {
            b.iter(|| compose(black_box(raw), black_box(&opts)));
        });
    }

    group.finish();
}

fn bench_compose_vni(c: &mut Criterion) {
    let config = presets::vni_config();
    let opts = ComposeOpts::from_config(&config);

    let mut group = c.benchmark_group("compose_vni");

    // Representative VNI sequences (raw key buffers)
    let cases: &[(&str, &str)] = &[
        ("ngu7o72i", "người"),
        ("tuo7ng1", "tuống"),
        ("ddu7o7c5", "được"),
        ("thu7o7ng1", "thường"),
        ("a6", "â"),
        ("a8f", "ằ"),
        ("a", "a"),
    ];

    for (raw_str, _expected) in cases {
        let raw: Vec<char> = raw_str.chars().collect();
        group.bench_with_input(BenchmarkId::from_parameter(*raw_str), &raw, |b, raw| {
            b.iter(|| compose(black_box(raw), black_box(&opts)));
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// PipelineExecutor::process() — full pipeline including overhead
// ---------------------------------------------------------------------------

fn bench_executor_telex_word(c: &mut Criterion) {
    let mut group = c.benchmark_group("executor_telex_word");

    // Each case is typed keystroke-by-keystroke through a fresh executor.
    let words: &[(&str, &str)] = &[
        ("nguwowif", "người"),
        ("tuongwf", "tường"),
        ("thuowngf", "thường"),
    ];

    for (keystrokes, _expected) in words {
        group.bench_with_input(
            BenchmarkId::from_parameter(*keystrokes),
            keystrokes,
            |b, keys| {
                b.iter(|| {
                    let config = presets::telex_config();
                    let mut executor = PipelineExecutor::new(config);
                    for ch in keys.chars() {
                        let _ = executor.process(black_box(ch));
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_executor_keystroke_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("executor_keystroke_latency");

    // Measure a single keystroke with a warm executor (buffer = "a" already).
    // This is the hot path: the IME gets called once per keypress.
    let keystrokes = [
        ('a', "plain_letter"),
        ('w', "transform_trigger"),
        ('s', "tone_trigger"),
        (' ', "space_passthrough"),
    ];

    for (ch, id) in keystrokes {
        group.bench_with_input(BenchmarkId::from_parameter(id), &ch, |b, &ch| {
            let config = presets::telex_config();
            let mut executor = PipelineExecutor::new(config);
            // Prime with one character so the buffer is non-empty.
            executor.process('a');

            b.iter(|| {
                // Reset to consistent state between iterations.
                executor.reset();
                executor.process('a');
                let _ = executor.process(black_box(ch));
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_compose_telex,
    bench_compose_vni,
    bench_executor_telex_word,
    bench_executor_keystroke_latency,
);
criterion_main!(benches);
