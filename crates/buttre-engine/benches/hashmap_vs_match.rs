//! Benchmark comparing HashMap vs Match for transform lookup
//!
//! This measures the performance difference between:
//! - find_transformation() - HashMap lookup
//! - find_transformation_optimized() - Match statement

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// Mock simplified transform functions for isolated testing
fn find_with_hashmap(last: char, input: char) -> Option<&'static str> {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("aa", "â");
    map.insert("aw", "ă");
    map.insert("dd", "đ");
    map.insert("ee", "ê");
    map.insert("oo", "ô");
    map.insert("ow", "ơ");
    map.insert("uw", "ư");
    
    let sequence = format!("{}{}", last, input);
    map.get(sequence.as_str()).copied()
}

fn find_with_match(last: char, input: char) -> Option<&'static str> {
    match (last.to_ascii_lowercase(), input.to_ascii_lowercase()) {
        ('a', 'a') => Some("â"),
        ('a', 'w') => Some("ă"),
        ('d', 'd') => Some("đ"),
        ('e', 'e') => Some("ê"),
        ('o', 'o') => Some("ô"),
        ('o', 'w') => Some("ơ"),
        ('u', 'w') => Some("ư"),
        _ => None,
    }
}

fn bench_lookup_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_method");
    
    let test_cases = vec![
        ('a', 'a'),
        ('a', 'w'),
        ('d', 'd'),
        ('e', 'e'),
        ('o', 'o'),
        ('o', 'w'),
        ('u', 'w'),
    ];
    
    for (last, input) in test_cases {
        let name = format!("{}{}", last, input);
        
        // Benchmark HashMap
        group.bench_with_input(
            BenchmarkId::new("HashMap", &name),
            &(last, input),
            |b, (l, i)| {
                b.iter(|| {
                    find_with_hashmap(black_box(*l), black_box(*i))
                });
            },
        );
        
        // Benchmark Match
        group.bench_with_input(
            BenchmarkId::new("Match", &name),
            &(last, input),
            |b, (l, i)| {
                b.iter(|| {
                    find_with_match(black_box(*l), black_box(*i))
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_lookup_comparison);
criterion_main!(benches);
