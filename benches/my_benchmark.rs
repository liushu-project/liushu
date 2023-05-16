use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use liushu_core::{
    dirs::PROJECT_DIRS,
    engine::{Engine, InputMethodEngine},
};

pub fn engine_benchmark(c: &mut Criterion) {
    let engine = Engine::init(&PROJECT_DIRS).unwrap();
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
