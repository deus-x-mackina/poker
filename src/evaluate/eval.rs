use std::fmt;

use crate::{
    evaluate::{hand_rank::PokerHandRank, meta::Meta},
    EvalClass,
};

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
/// Instances of `Eval` can be compared against one another in order to
/// determine which hand evaluations are better, worse, or equivalent. This can
/// be accomplished through the [`is_better_than`](Eval::is_better_than),
/// [`is_worse_than`](Eval::is_worse_than), and
/// [`is_equal_to`](Eval::is_equal_to) methods. `Eval` also implements [`Ord`]
/// and [`Eq`], so you may also use the operators `>`, `<`, and `==`
/// respectively. Card hand comparisons are robust are take into account kicker
/// cards, so hands with equivalent classes (i.e., two Jack-high flushes) can
/// still be compared.
///
/// ```
/// # fn main() {
/// #     if run().is_err() { std::process::exit(1); }
/// # }
/// #
/// # fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use poker::{card::Rank, cards, Card, EvalClass, Evaluator};
/// let eval = Evaluator::new();
/// let better_flush_cards: Vec<Card> = cards!("Jh Th 7h 4h 3h").try_collect()?;
/// let worse_flush_cards: Vec<Card> = cards!("Jc Tc 7c 4c 2c").try_collect()?;
///
/// let better_flush_hand = eval.evaluate(better_flush_cards)?;
/// let worse_flush_hand = eval.evaluate(worse_flush_cards)?;
///
/// let jack_high_flush = EvalClass::Flush {
///     high_rank: Rank::Jack,
/// };
/// assert_eq!(better_flush_hand.class(), jack_high_flush);
/// assert_eq!(worse_flush_hand.class(), jack_high_flush);
/// assert!(better_flush_hand.is_better_than(worse_flush_hand));
/// # Ok(())
/// # }
/// ```
/// [`Display`]: std::fmt::Display
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Eval(pub(crate) Meta);

impl Eval {
    /// The best possible poker hand, a royal flush.
    pub const BEST: Self = Self(Meta::BEST);
    /// The worst possible poker hand, a seven-high.
    pub const WORST: Self = Self(Meta::WORST);

    pub(crate) const fn hand_rank(self) -> PokerHandRank { self.0.hand_rank() }

    /// The class of poker hand that was evaluated. Useful for pattern matching
    /// as opposed to checking with an `is_x()` method.
    pub const fn class(self) -> EvalClass { self.0.class() }

    /// Compare this hand evaluation to another, returning `true` if this hand
    /// definitively beats the other, and `false` otherwise. This is equivalent
    /// to the operation `self > other`, but is a `const fn`.
    pub const fn is_better_than(self, other: Self) -> bool {
        self.hand_rank().is_better_than(other.hand_rank())
    }

    /// Compare this hand evaluation to another, returning `true` if this hand
    /// is definitively beaten by the other, and `false` otherwise. This is
    /// equivalent to the operation `self < other`, but is a `const fn`.
    pub const fn is_worse_than(self, other: Self) -> bool {
        self.hand_rank().is_worse_than(other.hand_rank())
    }

    /// Compare this hand evaluation to another, returning `true` if this hand
    /// is utterly equivalent to the other **in terms of its ranking in poker**,
    /// and `false` otherwise. This is equivalent to the operation `self ==
    /// other`, but is a `const fn`.
    pub const fn is_equal_to(self, other: Self) -> bool {
        self.hand_rank().0 == other.hand_rank().0
    }

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
    use crate::{cards, evaluate::tests::EVALUATOR};

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

    #[test]
    fn eval_better_worse_tie() {
        // Pair of twos
        let hand: Vec<_> = cards!["2h", "2c", "5h", "Qd", "6s"].try_collect().unwrap();
        let hand = EVALUATOR.evaluate(hand).unwrap();

        // Pair of threes
        let better: Vec<_> = cards!["3h", "3d", "5c", "Qs", "6h"].try_collect().unwrap();
        let better = EVALUATOR.evaluate(better).unwrap();

        // Ace high
        let worse: Vec<_> = cards!["Ac", "Kd", "Jd", "7h", "5d"].try_collect().unwrap();
        let worse = EVALUATOR.evaluate(worse).unwrap();

        let tie: Vec<_> = cards!["2s", "2d", "5s", "Qc", "6d"].try_collect().unwrap();
        let tie = EVALUATOR.evaluate(tie).unwrap();

        // `is_better_than`
        assert!(!hand.is_better_than(better));
        assert!(hand.is_better_than(worse));
        assert!(!hand.is_better_than(tie));

        // `is_worse_than`
        assert!(hand.is_worse_than(better));
        assert!(!hand.is_worse_than(worse));
        assert!(!hand.is_worse_than(tie));

        // `is_equal_to`
        assert!(!hand.is_equal_to(better));
        assert!(!hand.is_equal_to(worse));
        assert!(hand.is_equal_to(tie));
    }
}
