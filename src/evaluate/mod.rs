//! Anything and everything about evaluating poker hands.
//!
//! Given the specific format in which [`Card`] instances are coded, the key is
//! figuring out how to leverage that data representation to quickly evaluate
//! poker hands. (See [the `card` module] for more information).
//!
//! The key is the [`Evaluator`] structure, which dynamically generates some
//! internal [`HashMap`](std::collections::HashMap)s that it can use to quickly
//! look up poker hands by a hand's unique key, which is a product of prime
//! numbers that each [`Card`] instance codes.
//!
//! Because the [`Evaluator`] must dynamically generate its lookup tables at
//! *runtime*, and the tables are decently sized, it is recommended that:
//! - You instantiate an [`Evaluator`] as soon as possible
//! - You avoid cloning the [`Evaluator`]
//!
//! If there's going to be a performance bottleneck associated with this crate,
//! it will be making an [`Evaluator`] from scratch. Even so, in optimized
//! benching, [`Evaluator::new`] only takes about 300 - 400 *microseconds*
//! (there are 1 million microseconds in 1 second). Still, it is preferable
//! to be conservative here. All [`Evaluator`] methods borrow `Self` immutably,
//! so pass it around as you see fit.
//!
//! [`Card`]: crate::Card
//! [the `card` module`]: crate::card

#[macro_use]
mod evaluation;

mod class;
mod eval;
mod hand_rank;
// This needs to be public to bootstrap a lookup table in a build script, rather
// than shipping the `table.in` file, which is large and unnecessary
#[doc(hidden)]
pub mod lookup_table;
mod meta;
#[cfg(feature = "static_lookup")]
pub mod static_lookup;
mod utils;

#[doc(inline)]
pub use class::EvalClass;
#[doc(inline)]
pub use eval::Eval;

use crate::{card::Card, error::EvalError, evaluate::lookup_table::LookupTable};

/// This structure does all the heavy lifting of evaluating poker hands.
///
/// # Example
///
/// ```
/// use poker::{cards, Evaluator, EvalClass, Rank};
///
/// let eval = Evaluator::new();
///
/// let hand = cards!(
///     Four of Clubs,
///     Four of Spades,
///     Jack of Diamonds,
///     Jack of Clubs,
///     Jack of Hearts,
/// );
///
/// let result = eval.evaluate(&hand).expect("couldn't evaluate poker hand");
/// assert!(matches!(
///     result.class(),
///     EvalClass::FullHouse { pair: Rank::Four, trips: Rank::Jack }
/// ));
/// assert_eq!(
///     result.to_string(),
///     "Full house, jacks over fours"
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Evaluator(LookupTable);

impl Evaluator {
    /// Create a new [`Evaluator`]. Try to call this method only once and share
    /// the instance as much as possible.
    pub fn new() -> Self { Self(LookupTable::new()) }

    /// Evaluate a hand. This function takes anything that implements
    /// `AsRef<[Card]>`, so owned or borrowed slices of `Vec`s work fine
    /// here!
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
    /// [`EvalError`] for more.
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
    /// use poker::{cards, Card, Evaluator};
    ///
    /// const ROYAL_FLUSH: [Card; 5] = cards!(
    ///     Ten of Clubs,
    ///     Jack of Clubs,
    ///     Queen of Clubs,
    ///     King of Clubs,
    ///     Ace of Clubs,
    /// );
    /// let mut results = Vec::new();
    /// let eval = Evaluator::new();
    /// // Pass a slice
    /// results.push(eval.evaluate(&ROYAL_FLUSH).expect("couldn't evaluate hand"));
    /// // Pass an owned vector
    /// results.push(
    ///     eval.evaluate(ROYAL_FLUSH.to_vec())
    ///         .expect("couldn't evaluate hand"),
    /// );
    /// assert!(results.into_iter().all(|result| result.is_royal_flush()));
    /// ```
    ///
    /// With a hand and a board:
    ///
    /// ```
    /// use poker::{box_cards, cards, Card, EvalClass, Evaluator};
    ///
    /// let eval = Evaluator::new();
    /// let board: Vec<Card> = cards!("3c 5c As Jc Qh")
    ///     .try_collect()
    ///     .expect("couldn't parse cards");
    /// let hand: Vec<Card> = cards!("Tc Ac").try_collect().expect("couldn't parse cards");
    ///
    /// let result = eval
    ///     .evaluate(box_cards!(board, hand))
    ///     .expect("couldn't evaluate hand");
    /// assert!(matches!(result.class(), EvalClass::Flush { .. }));
    /// ```
    ///
    /// [`box_cards!`]: crate::box_cards
    pub fn evaluate<C: AsRef<[Card]>>(&self, cards: C) -> Result<Eval, EvalError> {
        let cards = cards.as_ref();
        evaluation::evaluate(self, cards)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::HashSet;

    use lazy_static::lazy_static;

    use super::*;
    use crate::{
        card::Card,
        deck,
        evaluate::{hand_rank::PokerHandRank, utils},
    };

    lazy_static! {
        pub static ref EVALUATOR: Evaluator = Evaluator::new();
    }

    #[test]
    fn test_all_five_card_combos() {
        let deck = deck::generate().collect::<Vec<_>>();
        let gen = utils::const_combos::<_, 5>(&deck);
        let evals = gen.fold(HashSet::with_capacity(7462), |mut ints, hand| {
            ints.insert(EVALUATOR.evaluate(&hand).unwrap());
            ints
        });
        assert_eq!(evals.len(), 7462);
        (1..=7462).for_each(move |i| {
            assert!(evals
                .iter()
                .any(|meta| meta.hand_rank() == PokerHandRank(i)));
        });
    }

    #[test]
    fn representative_five_card_hands() {
        representative_hand_evaluates_correctly::<FiveCardHand>(5);
    }

    #[test]
    fn representative_six_card_hands() {
        representative_hand_evaluates_correctly::<SixCardHand>(6);
    }

    #[test]
    fn representative_seven_card_hands() {
        representative_hand_evaluates_correctly::<SevenCardHand>(7);
    }

    fn representative_hand_evaluates_correctly<T: RepresentativeHand>(hand_size: usize) {
        assert!(T::ALL_HANDS.iter().all(|&hand| hand.len() == hand_size));

        let mut evaluations = T::ALL_HANDS.iter().map(|&hand| {
            let cards = Card::parse_to_iter(hand).try_collect::<Box<_>>().unwrap();
            EVALUATOR.evaluate(cards).unwrap()
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

    type Hand = &'static [&'static str];

    pub trait RepresentativeHand {
        const HIGH_CARD: Hand;
        const PAIR: Hand;
        const TWO_PAIR: Hand;
        const THREE_OF_A_KIND: Hand;
        const STRAIGHT: Hand;
        const FLUSH: Hand;
        const FULL_HOUSE: Hand;
        const FOUR_OF_A_KIND: Hand;
        const STRAIGHT_FLUSH: Hand;
        const ROYAL_FLUSH: Hand;

        const ALL_HANDS: &'static [Hand] = &[
            Self::HIGH_CARD,
            Self::PAIR,
            Self::TWO_PAIR,
            Self::THREE_OF_A_KIND,
            Self::STRAIGHT,
            Self::FLUSH,
            Self::FULL_HOUSE,
            Self::FOUR_OF_A_KIND,
            Self::STRAIGHT_FLUSH,
            Self::ROYAL_FLUSH,
        ];
    }

    pub struct FiveCardHand;

    #[rustfmt::skip]
    impl RepresentativeHand for FiveCardHand {
        const HIGH_CARD: Hand = &["Ah", "8s", "6d", "4c", "2h"];
        const PAIR: Hand = &["Ac", "Ah", "9s", "8d", "7c"];
        const TWO_PAIR: Hand = &["Kd", "Kc", "Qh", "Qs", "Jd"];
        const THREE_OF_A_KIND: Hand = &["Ac", "Ah", "As", "2d", "7c"];
        const STRAIGHT: Hand = &["5h", "6c", "7d", "8s", "9h"];
        const FLUSH: Hand = &["Ac", "5c", "Tc", "Jc", "8c"];
        const FULL_HOUSE: Hand = &["As", "Ad", "Ac", "Kh", "Ks"];
        const FOUR_OF_A_KIND: Hand = &["Ac", "Ah", "As", "Ad", "2h"];
        const STRAIGHT_FLUSH: Hand = &["5s", "6s", "7s", "8s", "9s"];
        const ROYAL_FLUSH: Hand = &["Th", "Jh", "Qh", "Kh", "Ah"];
    }

    pub struct SixCardHand;

    #[rustfmt::skip]
    impl RepresentativeHand for SixCardHand {
        const HIGH_CARD: &'static [&'static str] = &["Ah", "8s", "6d", "4c", "2h", "Jh"];
        const PAIR: &'static [&'static str] = &["Ac", "Ah", "9s", "8d", "7c", "6c"];
        const TWO_PAIR: &'static [&'static str] = &["Kd", "Kc", "Qh", "Qs", "Jd", "2c"];
        const THREE_OF_A_KIND: &'static [&'static str] = &["Ac", "Ah", "As", "2d", "7c", "3h"];
        const STRAIGHT: &'static [&'static str] = &["5h", "6c", "7d", "8s", "9h", "2d"];
        const FLUSH: &'static [&'static str] = &["Ac", "5c", "Tc", "Jc", "8c", "4h"];
        const FULL_HOUSE: &'static [&'static str] = &["As", "Ad", "Ac", "Kh", "Ks", "2d"];
        const FOUR_OF_A_KIND: &'static [&'static str] = &["Ac", "Ah", "As", "Ad", "2h", "3c"];
        const STRAIGHT_FLUSH: &'static [&'static str] = &["5s", "6s", "7s", "8s", "9s", "Ts"];
        const ROYAL_FLUSH: &'static [&'static str] = &["Th", "Jh", "Qh", "Kh", "Ah", "2c"];
    }

    pub struct SevenCardHand;

    #[rustfmt::skip]
    impl RepresentativeHand for SevenCardHand {
        const HIGH_CARD: &'static [&'static str] = &["Ah", "8s", "6d", "4c", "2h", "Jh", "Ts"];
        const PAIR: &'static [&'static str] = &["Ac", "Ah", "9s", "8d", "7c", "6c", "2h"];
        const TWO_PAIR: &'static [&'static str] = &["Kd", "Kc", "Qh", "Qs", "Jd", "2c", "3s"];
        const THREE_OF_A_KIND: &'static [&'static str] =
            &["Ac", "Ah", "As", "2d", "7c", "3h", "5s"];
        const STRAIGHT: &'static [&'static str] = &["5h", "6c", "7d", "8s", "9h", "2d", "Ac"];
        const FLUSH: &'static [&'static str] = &["Ac", "5c", "Tc", "Jc", "8c", "4h", "As"];
        const FULL_HOUSE: &'static [&'static str] = &["As", "Ad", "Ac", "Kh", "Ks", "2d", "3c"];
        const FOUR_OF_A_KIND: &'static [&'static str] = &["Ac", "Ah", "As", "Ad", "2h", "3c", "4d"];
        const STRAIGHT_FLUSH: &'static [&'static str] = &["5s", "6s", "7s", "8s", "9s", "Ts", "2c"];
        const ROYAL_FLUSH: &'static [&'static str] = &["Th", "Jh", "Qh", "Kh", "Ah", "2c", "3s"];
    }
}
