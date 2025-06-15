use std::ops::{BitAnd, BitOr, Index};

use super::poker_type::{FiveCard, ThreeCard};
use crate::{
    Card, EvalError,
    error::EvalResult,
    evaluate::{eval::Eval, utils},
};

pub trait Evaluation {
    type LookupFive: for<'a> Index<&'a i32, Output = Eval<FiveCard>>;
    type LookupThree: for<'a> Index<&'a i32, Output = Eval<ThreeCard>>;
    fn flush_lookup_five(&self) -> &Self::LookupFive;
    fn unsuited_lookup_five(&self) -> &Self::LookupFive;
    fn flush_lookup_three(&self) -> &Self::LookupThree;
    fn unsuited_lookup_three(&self) -> &Self::LookupThree;
}

pub fn evaluate_three(evaluator: &impl Evaluation, cards: &[Card]) -> EvalResult<ThreeCard> {
    if !utils::all_unique(cards) {
        return Err(EvalError::CardsNotUnique(cards.to_vec()));
    }

    match cards {
        &[a, b, c] => Ok(three(evaluator, [a, b, c])),
        _ => Err(EvalError::InvalidHandSize3(cards.len())),
    }
}

fn three(evaluator: &impl Evaluation, cards: [Card; 3]) -> Eval<ThreeCard> {
    let uniques = cards.map(Card::unique_integer);
    let detect_flush = uniques.iter().fold(0xF000, i32::bitand) != 0;
    if detect_flush {
        let bit_rank_or = uniques.iter().fold(0, i32::bitor) >> 16;
        let prime = utils::prime_product_from_rank_bits(bit_rank_or as i16);
        evaluator.flush_lookup_three()[&prime]
    } else {
        let prime = utils::prime_product_from_hand(cards);
        evaluator.unsuited_lookup_three()[&prime]
    }
}

pub fn evaluate_five(evaluator: &impl Evaluation, cards: &[Card]) -> EvalResult<FiveCard> {
    if !utils::all_unique(cards) {
        return Err(EvalError::CardsNotUnique(cards.to_vec()));
    }

    match *cards {
        [a, b, c, d, e] => Ok(five(evaluator, [a, b, c, d, e])),
        [_, _, _, _, _, ..] => Ok(six_plus(evaluator, cards)),
        _ => Err(EvalError::InvalidHandSize5(cards.len())),
    }
}

fn five(evaluator: &impl Evaluation, cards: [Card; 5]) -> Eval<FiveCard> {
    let uniques = cards.map(Card::unique_integer);

    let detect_flush = uniques.iter().fold(0xF000, i32::bitand) != 0;

    if detect_flush {
        let bit_rank_or = uniques.iter().fold(0, i32::bitor) >> 16;
        let prime = utils::prime_product_from_rank_bits(bit_rank_or as i16);
        evaluator.flush_lookup_five()[&prime]
    } else {
        let prime = utils::prime_product_from_hand(cards);
        evaluator.unsuited_lookup_five()[&prime]
    }
}

fn six_plus(evaluator: &impl Evaluation, cards: &[Card]) -> Eval<FiveCard> {
    debug_assert!(cards.len() > 5);
    let mut current_max = Eval::<FiveCard>::WORST;
    let all_five_card_combos = utils::const_combos::<_, 5>(cards);
    for combo in all_five_card_combos {
        let score = five(evaluator, combo);
        if score > current_max {
            current_max = score;
        }
    }
    current_max
}
