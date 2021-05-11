use criterion::{Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use poker::{evaluate::static_lookup, Card, Evaluator};

fn bench_evaluator(c: &mut Criterion) {
    c.bench_function("Evaluator::new()", |b| b.iter(Evaluator::new));
}

fn bench_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval");
    group.sample_size(10);
    let eval = Evaluator::new();
    let gen = Card::generate_deck().combinations(5).collect::<Box<_>>();

    let routine = {
        let eval = eval;
        let gen = &gen;
        move || {
            for cards in gen.iter() {
                let _ = eval.evaluate(cards);
            }
        }
    };

    group.bench_function("dynamic", |b| {
        b.iter(&routine)
    });

    group.bench_function("static", |b| {
        b.iter(|| {
            for cards in gen.iter() {
                let _ = static_lookup::evaluate(cards);
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_evaluator,
    bench_eval
);

criterion_main!(benches);
