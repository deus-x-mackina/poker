use criterion::{Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use poker::{Evaluator, cards, deck, evaluate::static_lookup};

fn bench_evaluator(c: &mut Criterion) {
    c.bench_function("Evaluator::new()", |b| b.iter(Evaluator::new));
}

fn bench_single_5card_hand_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_5card_hand_eval");

    let eval = Evaluator::new();
    let hand: Vec<_> = cards!("Th", "Jh", "Qh", "Kh", "Ah").try_collect().unwrap();

    group.bench_function("dynamic", |b| {
        b.iter(|| {
            let _ = eval.evaluate_five(&hand);
        })
    });

    group.bench_function("static", |b| {
        b.iter(|| {
            let _ = static_lookup::evaluate_five(&hand);
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
            let _ = eval.evaluate_five(&hand);
        })
    });

    group.bench_function("static", |b| {
        b.iter(|| {
            let _ = static_lookup::evaluate_five(&hand);
        })
    });

    group.finish();
}

fn bench_all_5card_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_5card_eval");
    group.sample_size(10);
    let eval = Evaluator::new();
    let generator = deck::generate().combinations(5).collect::<Box<_>>();

    let routine = {
        let generator = &generator;
        move || {
            for cards in generator.iter() {
                let _ = eval.evaluate_five(cards);
            }
        }
    };

    group.bench_function("dynamic", |b| b.iter(&routine));

    group.bench_function("static", |b| {
        b.iter(|| {
            for cards in generator.iter() {
                let _ = static_lookup::evaluate_five(cards);
            }
        });
    });

    group.finish();
}

fn bench_single_3card_hand_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_3card_hand_eval");

    let eval = Evaluator::new();
    let hand: Vec<_> = cards!("Ah", "Kh", "Qh").try_collect().unwrap();

    group.bench_function("dynamic", |b| {
        b.iter(|| {
            let _ = eval.evaluate_three(&hand);
        })
    });

    group.bench_function("static", |b| {
        b.iter(|| {
            let _ = static_lookup::evaluate_three(&hand);
        })
    });

    group.finish();
}

fn bench_all_3card_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_3card_eval");
    group.sample_size(10);
    let eval = Evaluator::new();
    let generator = deck::generate().combinations(3).collect::<Box<_>>();

    let routine = {
        let generator = &generator;
        move || {
            for cards in generator.iter() {
                let _ = eval.evaluate_three(cards);
            }
        }
    };

    group.bench_function("dynamic", |b| b.iter(&routine));

    group.bench_function("static", |b| {
        b.iter(|| {
            for cards in generator.iter() {
                let _ = static_lookup::evaluate_three(cards);
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
    bench_all_5card_eval,
    bench_single_3card_hand_eval,
    bench_all_3card_eval,
);

criterion_main!(benches);
