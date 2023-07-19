use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use poker::{cards, evaluate::static_lookup, Card, Evaluator};

fn bench_evaluator(c: &mut Criterion) {
    c.bench_function("Evaluator::new()", |b| b.iter(Evaluator::new));
}

fn bench_single_5card_hand_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_5card_hand_eval");

    let eval = Evaluator::new();
    let hand: Vec<_> = cards!("Th", "Jh", "Qh", "Kh", "Ah").try_collect().unwrap();

    group.bench_function("dynamic", |b| {
        b.iter(|| {
            let _ = eval.evaluate(&hand);
        })
    });

    group.bench_function("static", |b| {
        b.iter(|| {
            let _ = static_lookup::evaluate(&hand);
        })
    });
    
    group.finish();
}

fn bench_single_7card_hand_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_7card_hand_eval");

    let eval = Evaluator::new();
    let hand: Vec<_> = cards!("8h", "9h", "Th", "Jh", "Qh", "Kh", "Ah")
        .try_collect()
        .unwrap();

    group.bench_function("dynamic", |b| {
        b.iter(|| {
            let _ = eval.evaluate(&hand);
        })
    });

    group.bench_function("static", |b| {
        b.iter(|| {
            let _ = static_lookup::evaluate(&hand);
        })
    });
    
    group.finish();
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

    group.bench_function("dynamic", |b| b.iter(&routine));

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
    bench_single_5card_hand_eval,
    bench_single_7card_hand_eval,
    bench_eval
);

criterion_main!(benches);
