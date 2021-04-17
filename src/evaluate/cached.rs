//! TODO

use crate::{evaluate::utils, ext::AllUnique, Card, Eval, EvalError};

mod statics {
    include!("../../table.in");
}

/// TODO
pub fn evaluate<C: AsRef<[Card]>>(cards: C) -> Result<Eval, EvalError> {
    let cards = cards.as_ref();
    if cards.all_unique() {
        match cards.len() {
            x if x < 5 => Err(EvalError::InvalidHandSize(x)),
            5 => Ok(five(cards)),
            _ => Ok(six_plus(cards)),
        }
    } else {
        Err(EvalError::CardsNotUnique(cards.to_vec()))
    }
}

fn five(cards: &[Card]) -> Eval {
    debug_assert_eq!(cards.len(), 5);
    let detect_flush = cards
        .iter()
        .fold(0xF000, |acc, card| acc & card.unique_integer())
        != 0;

    if detect_flush {
        let bit_rank_or = cards
            .iter()
            .fold(0, |acc, card| acc | card.unique_integer())
            >> 16;
        let prime = utils::prime_product_from_rank_bits(bit_rank_or as i16);
        Eval(statics::FLUSH_LOOKUP[&prime])
    } else {
        let prime = utils::prime_product_from_hand(cards);
        Eval(statics::UNSUITED_LOOKUP[&prime])
    }
}

fn six_plus(cards: &[Card]) -> Eval {
    debug_assert!(cards.len() > 5);
    let mut current_max = Eval::WORST;
    let all_five_card_combos = utils::combinations_generator(cards.iter().cloned(), 5);
    for combo in all_five_card_combos {
        let score = five(&combo);
        if score > current_max {
            current_max = score;
        }
    }
    current_max
}
