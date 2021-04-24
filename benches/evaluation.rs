use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker::{Evaluator, evaluate::static_lookup, Card};
use itertools::Itertools;

fn bench_evaluator(c: &mut Criterion) {
    c.bench_function("Evaluator::new()", |b| b.iter(black_box(Evaluator::new)));
}

fn bench_dynamic_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("dynamic-eval");
    group.sample_size(10);
    let eval = Evaluator::new();
    let gen = Card::generate_deck().combinations(5).collect::<Box<_>>();
    group.bench_function("Evaluator evaulation", |b| {
        b.iter(|| for cards in gen.iter() {
            eval.evaluate(cards).unwrap();
        })
    });
    group.finish();
}

fn bench_static_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("static-eval");
    group.sample_size(10);
    let gen = Card::generate_deck().combinations(5).collect::<Box<_>>();
    group.bench_function("Static evaluation", |b| {
        b.iter(|| for cards in gen.iter() {
            static_lookup::evaluate(cards).unwrap();
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_evaluator,
    bench_dynamic_eval,
    bench_static_eval
);

criterion_main!(benches);
