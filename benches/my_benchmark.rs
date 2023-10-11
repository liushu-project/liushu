use std::io::stdin;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use liushu_core::engine::{Engine, InputMethodEngine};

pub fn engine_benchmark(c: &mut Criterion) {
    println!("Enter engine dict path:");
    let mut dict_path = String::new();
    stdin().read_line(&mut dict_path).unwrap();

    let engine = Engine::new(dict_path.trim()).unwrap();
    let test_inputs = ["a", "aac", "bo", "cfl", "df", "fojq", "qiq", "hir", "zzz"];

    let mut group = c.benchmark_group("Engine bench");
    for input in test_inputs {
        group.bench_with_input(BenchmarkId::new("Formula<sunman>", input), input, |b, i| {
            b.iter(|| {
                engine.search(i).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, engine_benchmark);
criterion_main!(benches);
