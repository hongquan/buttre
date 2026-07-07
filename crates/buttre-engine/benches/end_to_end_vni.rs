//! End-to-end VNI performance benchmark
//! Measures real-world typing scenarios

use buttre_engine::pipeline::presets;
use buttre_engine::pipeline::PipelineExecutor;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Benchmark real Vietnamese words typed in VNI
fn bench_vni_real_words(c: &mut Criterion) {
    let mut group = c.benchmark_group("vni_real_words");

    let test_words = vec![
        ("Vie65t", "Việt", "viet"),
        ("ngu7o72i", "người", "nguoi"),
        ("thu7o7ng", "thương", "thuong"),
        ("tru7o72ng", "trường", "truong"),
        ("ba3n", "bản", "ban"),
        ("to6i", "tôi", "toi"),
        ("co1", "có", "co"),
        ("la2", "là", "la"),
        ("mo65t", "một", "mot"),
        ("kho5ng", "không", "khong"),
    ];

    for (input, _expected, id) in test_words {
        group.bench_with_input(BenchmarkId::from_parameter(id), &input, |b, input_str| {
            b.iter(|| {
                let config = presets::vni_config();
                let mut executor = PipelineExecutor::new(config);

                for ch in input_str.chars() {
                    let _actions = executor.process(black_box(ch));
                }
            });
        });
    }

    group.finish();
}

/// Benchmark keystroke latency (per-character processing time)
fn bench_vni_keystroke_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("vni_keystroke_latency");

    // Test individual keystrokes
    let keystrokes = vec![
        ('a', "letter_a"),
        ('6', "transform_6"),
        ('1', "tone_1"),
        ('đ', "already_transformed"),
        (' ', "space"),
    ];

    for (ch, id) in keystrokes {
        group.bench_with_input(BenchmarkId::from_parameter(id), &ch, |b, &ch| {
            let config = presets::vni_config();
            let mut executor = PipelineExecutor::new(config);

            b.iter(|| {
                executor.reset();
                let _actions = executor.process(black_box(ch));
            });
        });
    }

    group.finish();
}

/// Benchmark sentence processing (multiple words)
fn bench_vni_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("vni_sentence");

    // Real Vietnamese sentences in VNI
    let sentences = vec![
        (
            "To6i la2 mo65t ngu7o72i Vie65t Nam",
            "Tôi là một người Việt Nam",
            "simple_sentence",
        ),
        (
            "Chu1ng to6i dang ho5c tie65ng Vie65t",
            "Chúng tôi đang học tiếng Việt",
            "learning_sentence",
        ),
    ];

    for (input, _expected, id) in sentences {
        group.bench_with_input(BenchmarkId::from_parameter(id), &input, |b, input_str| {
            b.iter(|| {
                let config = presets::vni_config();
                let mut executor = PipelineExecutor::new(config);

                for ch in input_str.chars() {
                    let _actions = executor.process(black_box(ch));
                }
            });
        });
    }

    group.finish();
}

/// Benchmark worst-case scenarios
fn bench_vni_worst_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("vni_worst_case");

    // Worst case: Many transforms and tones
    group.bench_function("complex_word", |b| {
        let input = "qua74n"; // quận
        b.iter(|| {
            let config = presets::vni_config();
            let mut executor = PipelineExecutor::new(config);

            for ch in input.chars() {
                let _actions = executor.process(black_box(ch));
            }
        });
    });

    // Worst case: Number confusion
    group.bench_function("windows_10", |b| {
        let input = "Windows 10";
        b.iter(|| {
            let config = presets::vni_config();
            let mut executor = PipelineExecutor::new(config);

            for ch in input.chars() {
                let _actions = executor.process(black_box(ch));
            }
        });
    });

    group.finish();
}

/// Benchmark memory allocations
fn bench_vni_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vni_allocations");

    group.bench_function("new_executor", |b| {
        b.iter(|| {
            let config = presets::vni_config();
            let _executor = PipelineExecutor::new(black_box(config));
        });
    });

    group.bench_function("reset_executor", |b| {
        let config = presets::vni_config();
        let mut executor = PipelineExecutor::new(config);

        b.iter(|| {
            executor.reset();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_vni_real_words,
    bench_vni_keystroke_latency,
    bench_vni_sentence,
    bench_vni_worst_case,
    bench_vni_allocations
);
criterion_main!(benches);
