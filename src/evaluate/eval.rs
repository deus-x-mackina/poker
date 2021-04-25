use std::fmt;

#[cfg(test)]
use super::hand_rank::PokerHandRank;
use crate::{evaluate::meta::Meta, EvalClass};

/// The result of a successful poker hand evaluation. When printed in
/// [`Display`] format, shows the proper, qualified name of the poker hand.
///
/// # Example
///
/// ```
/// use poker::{cards, Evaluator};
/// let hand = cards!("7c 8d 3s Ks 7h")
///     .try_collect::<Vec<_>>()
///     .expect("couldn't parse cards");
/// let eval = Evaluator::new();
/// let result = eval.evaluate(hand).expect("couldn't evaluate hand");
/// assert_eq!(result.to_string(), "Pair, sevens");
/// assert!(result.is_pair());
/// ```
///
/// [`Display`]: std::fmt::Display
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Eval(pub(crate) Meta);

impl Eval {
    /// The best possible poker hand, a royal flush.
    pub const BEST: Self = Self(Meta::BEST);
    /// The worst possible poker hand, a seven-high.
    pub const WORST: Self = Self(Meta::WORST);

    #[cfg(test)]
    pub(crate) const fn hand_rank(self) -> PokerHandRank { self.0.hand_rank() }

    /// The class of poker hand that was evaluated. Useful for pattern matching
    /// as opposed to checking with an `is_x()` method.
    pub const fn class(self) -> EvalClass { self.0.class() }

    /// Check whether this hand is a high-card.
    pub const fn is_high_card(self) -> bool { self.0.is_high_card() }

    /// Check whether this hand is a pair.
    pub const fn is_pair(self) -> bool { self.0.is_pair() }

    /// Check whether this hand is a two-pair.
    pub const fn is_two_pair(self) -> bool { self.0.is_two_pair() }

    /// Check whether this hand is a three-of-a-kind.
    pub const fn is_three_of_a_kind(self) -> bool { self.0.is_three_of_a_kind() }

    /// Check whether this hand is a straight.
    pub const fn is_straight(self) -> bool { self.0.is_straight() }

    /// Check whether this hand is a flush.
    pub const fn is_flush(self) -> bool { self.0.is_flush() }

    /// Check whether this hand is a full house.
    pub const fn is_full_house(self) -> bool { self.0.is_full_house() }

    /// Check whether this hand is a four-of-a-kind.
    pub const fn is_four_of_a_kind(self) -> bool { self.0.is_four_of_a_kind() }

    /// Check whether this hand is a straight flush.
    pub const fn is_straight_flush(self) -> bool { self.0.is_straight_flush() }

    /// Check whether this hand is a royal flush.
    pub const fn is_royal_flush(self) -> bool { self.0.is_royal_flush() }
}

impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_best() {
        let result = Eval::BEST;
        assert!(result.is_straight_flush());
        assert!(result.is_royal_flush());
        assert_eq!(result.to_string(), "Royal flush");
    }

    #[test]
    fn eval_worst() {
        let result = Eval::WORST;
        assert!(result.is_high_card());
        assert_eq!(result.to_string(), "High card, seven");
    }
}
