use std::ops::Index;

use rustc_hash::FxHashMap;

use crate::{
    evaluate::{meta::Meta, utils},
    Card, Eval, EvalError,
};

pub trait Evaluation {
    type Lookup: for<'a> Index<&'a i32, Output = Meta>;
    fn flush_lookup(&self) -> &Self::Lookup;
    fn unsuited_lookup(&self) -> &Self::Lookup;
}

impl Evaluation for super::Evaluator {
    type Lookup = FxHashMap<i32, Meta>;

    fn flush_lookup(&self) -> &Self::Lookup { &self.0.flush_lookup }

    fn unsuited_lookup(&self) -> &Self::Lookup { &self.0.unsuited_lookup }
}

pub fn evaluate(evaluator: &impl Evaluation, cards: &[Card]) -> Result<Eval, EvalError> {
    if utils::all_unique(cards) {
        match cards.len() {
            x if x < 5 => Err(EvalError::InvalidHandSize(x)),
            5 => {
                let cards_array = [cards[0], cards[1], cards[2], cards[3], cards[4]];
                Ok(five(evaluator, cards_array))
            }
            _ => Ok(six_plus(evaluator, cards)),
        }
    } else {
        Err(EvalError::CardsNotUnique(cards.to_vec()))
    }
}

fn five(evaluator: &impl Evaluation, cards: [Card; 5]) -> Eval {
    let uniques = cards.map(Card::unique_integer);

    let detect_flush = uniques.into_iter().fold(0xF000, |acc, x| acc & x) != 0;

    if detect_flush {
        let bit_rank_or = uniques.into_iter().fold(0, |acc, x| acc | x) >> 16;
        let prime = utils::prime_product_from_rank_bits(bit_rank_or as i16);
        Eval(evaluator.flush_lookup()[&prime])
    } else {
        let prime = utils::prime_product_from_hand(cards);
        Eval(evaluator.unsuited_lookup()[&prime])
    }
}

fn six_plus(evaluator: &impl Evaluation, cards: &[Card]) -> Eval {
    debug_assert!(cards.len() > 5);
    let mut current_max = Eval::WORST;
    let all_five_card_combos = utils::const_combos::<_, 5>(cards);
    for combo in all_five_card_combos {
        let score = five(evaluator, combo);
        if score > current_max {
            current_max = score;
        }
    }
    current_max
}
