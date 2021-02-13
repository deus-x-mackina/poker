use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker::Evaluator;

fn bench_evaluator(c: &mut Criterion) {
    c.bench_function("Evaluator::new()", |b| b.iter(black_box(Evaluator::new)));
}

criterion_group!(benches, bench_evaluator);

criterion_main!(benches);
