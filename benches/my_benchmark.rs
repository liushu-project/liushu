use criterion::{black_box, criterion_group, criterion_main, Criterion};
use liushu_core::{
    dirs::PROJECT_DIRS,
    engine::{EngineWithRedb, InputMethodEngine, ShapeCodeEngine},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let sunman = ShapeCodeEngine::default();
    let sunman2 = EngineWithRedb::with(&PROJECT_DIRS.target_dir).unwrap();
    c.bench_function("engine with sqlite bench", |b| {
        b.iter(|| {
            sunman.search(black_box("hir")).unwrap();
        })
    });
    c.bench_function("engine with redb bench", |b| {
        b.iter(|| {
            sunman2.search(black_box("hir")).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
