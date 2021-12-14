use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path="../src/main.rs"]
mod main;

fn aggregated_counter(input: &str) -> usize {
    let mut puzzle = main::Puzzle::from_str(input);
    puzzle.step(40);
    puzzle.score()
}

fn memorized_counter(input: &str) -> usize {
    let mut puzzle = main::puzzle_dp::Puzzle::from_str(input);
    puzzle.score_depth(40)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark");
    group.sample_size(1000);
    let input: String = fs::read_to_string("input").unwrap();
    group.bench_function("aggregated", |b| b.iter(|| {
        aggregated_counter(black_box(&input))
    }));
    group.bench_function("memorized", |b| b.iter(|| {
        memorized_counter(black_box(&input))
    }));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);