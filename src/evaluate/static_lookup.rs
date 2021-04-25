//! This module is available under the non-default `static_lookup` feature and
//! offers similar functionality to the [`Evaluator`] type, but [`evaluate`]
//! comes as a free function. The main difference is that the [`evaluate`] uses
//! a static lookup table, built into the library.
//!
//! Because the `static` lookup table doesn't allocate any memory on the heap,
//! this module may become the foundation for providing `no_std` support in the
//! future.
//!
//! **Warning:** Enabling the `static_lookup` feature will greatly increase the
//! size of the resulting library.
//!
//! [`Evaluator`]: crate::Evaluator

use crate::{evaluate::utils, ext::AllUnique, Card, Eval, EvalError};

mod statics {
    include!("../../table.in");
}

/// Evaluate a hand using the static lookup table bundled with the library.
/// This function takes anything that implements `AsRef<[Card]>`, so owned or
/// borrowed slices of `Vec`s work fine here!
///
/// If you need to evaluate a hand in the context of a board (for example,
/// in Texas Holdem), you just need to combine both slices (such as with
/// [`box_cards!`]) and pass it to this method. See the exaples for
/// more.
///
/// # Errors
///
/// This function will fail if the total number of cards is less than five,
/// or if not all the cards passed in are unique. See
/// [`EvalError`](crate::EvalError) for more.
///
/// # Performance
///
/// Optimal performance is achieved with a set of 5, 6, or 7 cards. Hands
/// are evaulated using combinatorics to find the best 5-card
/// combination, so the more cards you pass to this method, the longer
/// it will take to evaluate.
///
/// # Example
///
/// ```
/// use poker::{cards, evaluate::static_lookup, Card};
///
/// const ROYAL_FLUSH: [Card; 5] = cards!(
///     Ten of Clubs,
///     Jack of Clubs,
///     Queen of Clubs,
///     King of Clubs,
///     Ace of Clubs,
/// );
/// let mut results = Vec::new();
/// // Pass a slice
/// results.push(static_lookup::evaluate(&ROYAL_FLUSH).expect("couldn't evaluate hand"));
/// // Pass an owned vector
/// results.push(static_lookup::evaluate(ROYAL_FLUSH.to_vec()).expect("couldn't evaluate hand"));
/// assert!(results.into_iter().all(|result| result.is_royal_flush()));
/// ```
///
/// With a hand and a board:
///
/// ```
/// use poker::{box_cards, cards, evaluate::static_lookup, Card, EvalClass};
///
/// let board: Vec<Card> = cards!("3c 5c As Jc Qh")
///     .try_collect()
///     .expect("couldn't parse cards");
/// let hand: Vec<Card> = cards!("Tc Ac").try_collect().expect("couldn't parse cards");
///
/// let result = static_lookup::evaluate(box_cards!(board, hand)).expect("couldn't evaluate hand");
/// assert!(matches!(result.class(), EvalClass::Flush { .. }));
/// ```
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        card::Card,
        evaluate::{
            hand_rank::PokerHandRank,
            meta::Meta,
            tests::{FiveCardHand, RepresentativeHand, SevenCardHand, SixCardHand},
            utils,
        },
    };

    #[test]
    fn test_all_five_card_combos() {
        let gen = utils::combinations_generator(Card::generate_deck(), 5);
        let evals = gen.fold(HashSet::with_capacity(7462), |mut ints, hand| {
            ints.insert(evaluate(&hand).unwrap());
            ints
        });
        assert_eq!(evals.len(), 7462);
        (1..=7462).for_each(|i| {
            assert!(evals
                .iter()
                .any(|meta| meta.hand_rank() == PokerHandRank(i)))
        });
    }

    fn representative_hand_evaluates_correctly<T: RepresentativeHand>() {
        let mut evaluations = T::ALL_HANDS.iter().map(|&hand| {
            let cards = Card::parse_to_iter(hand)
                .try_collect::<Box<_>>()
                .unwrap();

            evaluate(cards).unwrap()
        });

        assert!(evaluations.next().unwrap().is_high_card());
        assert!(evaluations.next().unwrap().is_pair());
        assert!(evaluations.next().unwrap().is_two_pair());
        assert!(evaluations.next().unwrap().is_three_of_a_kind());
        assert!(evaluations.next().unwrap().is_straight());
        assert!(evaluations.next().unwrap().is_flush());
        assert!(evaluations.next().unwrap().is_full_house());
        assert!(evaluations.next().unwrap().is_four_of_a_kind());
        assert!(evaluations.next().unwrap().is_straight_flush());
        assert!(evaluations.next().unwrap().is_royal_flush());
        assert!(evaluations.next().is_none());
    }

    #[test]
    fn representative_five_card_hands() {
        representative_hand_evaluates_correctly::<FiveCardHand>();
    }

    #[test]
    fn representative_six_card_hands() { representative_hand_evaluates_correctly::<SixCardHand>(); }

    #[test]
    fn representative_seven_card_hands() {
        representative_hand_evaluates_correctly::<SevenCardHand>();
    }
}
