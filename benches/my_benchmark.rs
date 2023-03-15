use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use liushu_core::{
    dirs::PROJECT_DIRS,
    engine::{EngineWithRedb, InputMethodEngine, ShapeCodeEngine},
};

pub fn engine_benchmark(c: &mut Criterion) {
    let sunman = ShapeCodeEngine::default();
    let sunman2 = EngineWithRedb::with(&PROJECT_DIRS.target_dir).unwrap();
    let test_inputs = ["a", "aac", "bo", "cfl", "df", "fojq", "qiq", "hir", "zzz"];

    let mut group = c.benchmark_group("Engine bench");
    for input in test_inputs {
        group.bench_with_input(
            BenchmarkId::new("Engine with sqlite", input),
            input,
            |b, i| {
                b.iter(|| {
                    sunman.search(i).unwrap();
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Engine with redb", input),
            input,
            |b, i| {
                b.iter(|| {
                    sunman2.search(i).unwrap();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, engine_benchmark);
criterion_main!(benches);
