//! This module provides the [`Evaluator`] struct, which is used to evaluate
//! poker hands. The [`Evaluator`] struct constructs lookup tables for
//! three-card and five-card poker hands upon creation.

use rustc_hash::FxHashMap;

use crate::{
    card::Card,
    error::EvalResult,
    evaluate::{
        eval::Eval,
        evaluation::{self, Evaluation},
        five_card::lookup_table::LookupTable as FiveCardTable,
        poker_type::{FiveCard, ThreeCard},
        three_card::lookup_table::LookupTable as ThreeCardTable,
    },
};

#[derive(Debug, PartialEq, Eq, Default)]
/// The `Evaluator` struct is used to evaluate poker hands.
/// It contains lookup tables for three-card and five-card poker hands.
///
/// After initialization, the lookup table no longer needs to be mutated,
/// meaning it is safe to share the evaluator across threads.
///
/// # Example
///
/// ```
/// use poker::{Evaluator, deck};
///
/// let evaluator = Evaluator::new();
/// let mut deck = deck::shuffled();
/// let hand = deck.drain(..5).collect::<Vec<_>>();
/// println!(
///     "{}",
///     evaluator
///         .evaluate_five(hand)
///         .expect("Failed to evaluate hand")
/// );
/// ```
pub struct Evaluator {
    /// Lookup table for three-card poker hands.
    three_card_table: ThreeCardTable,
    /// Lookup table for five-card poker hands.
    five_card_table: FiveCardTable,
}

impl Evaluator {
    /// Creates a new [`Evaluator`] with initialized lookup tables.
    pub fn new() -> Self {
        Self {
            three_card_table: ThreeCardTable::new(),
            five_card_table: FiveCardTable::new(),
        }
    }

    /// Evaluates a five-card poker hand.
    ///
    /// Note that this function can find the best five-card poker hand as long
    /// as *at least* five cards are provided.
    ///
    /// This function accepts any type implementing `AsRef<[Card]>`, such as
    /// `&[Card]` or `Vec<Card>`. You are free to pass in owned values or
    /// borrowed values.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Evaluator, cards, deck};
    ///
    /// let evaluator = Evaluator::new();
    /// let mut deck = deck::shuffled();
    ///
    /// // Must pass 5+ cards
    /// let too_few = deck.drain(..4).collect::<Vec<_>>();
    /// assert!(evaluator.evaluate_five(&too_few).is_err());
    ///
    /// // Cards must be unique
    /// let duplicates = cards!("As As As As As").try_collect::<Vec<_>>().unwrap();
    /// assert!(evaluator.evaluate_five(&duplicates).is_err());
    ///
    /// let hand = deck.drain(..5).collect::<Vec<_>>();
    /// assert!(evaluator.evaluate_five(hand).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// - The cards provided are not unique.
    /// - There are less than five cards provided.
    pub fn evaluate_five(&self, cards: impl AsRef<[Card]>) -> EvalResult<FiveCard> {
        evaluation::evaluate_five(self, cards.as_ref())
    }

    /// Evaluates a three-card poker hand.
    ///
    /// Note that this function can only work with groups of exactly 3 cards.
    ///
    /// This function accepts any type implementing `AsRef<[Card]>`, such as
    /// `&[Card]` or `Vec<Card>`. You are free to pass in owned values or
    /// borrowed values.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    ///
    /// // Must pass exactly 3 cards
    /// let too_few = cards!("As Ad").try_collect::<Vec<_>>().unwrap();
    /// assert!(evaluator.evaluate_three(&too_few).is_err());
    ///
    /// // Cards must be unique
    /// let duplicates = cards!("As As As").try_collect::<Vec<_>>().unwrap();
    /// assert!(evaluator.evaluate_three(&duplicates).is_err());
    ///
    /// let hand = cards!("As Ad Ac").try_collect::<Vec<_>>().unwrap();
    /// assert!(evaluator.evaluate_three(hand).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// - The cards provided are not unique.
    /// - There are not exactly three cards provided.
    pub fn evaluate_three(&self, cards: impl AsRef<[Card]>) -> EvalResult<ThreeCard> {
        evaluation::evaluate_three(self, cards.as_ref())
    }
}

impl Evaluation for Evaluator {
    type LookupFive = FxHashMap<i32, Eval<FiveCard>>;
    type LookupThree = FxHashMap<i32, Eval<ThreeCard>>;

    fn flush_lookup_five(&self) -> &Self::LookupFive { &self.five_card_table.flush_lookup }

    fn unsuited_lookup_five(&self) -> &Self::LookupFive { &self.five_card_table.unsuited_lookup }

    fn flush_lookup_three(&self) -> &Self::LookupThree { &self.three_card_table.flush_lookup }

    fn unsuited_lookup_three(&self) -> &Self::LookupThree { &self.three_card_table.unsuited_lookup }
}
