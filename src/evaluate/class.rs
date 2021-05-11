use std::fmt;

use crate::Rank;

/// A utility enumeration type for pattern matching against the result of
/// [`Eval::class`](super::Eval::class). Each variant represents a class of
/// poker hand. Royal flush is not included, but can be matched against
/// `EvalClass:StraightFlush { high_card: Rank::Ace }` if desired.
///
/// # Example
///
/// ```
/// use poker::{cards, Evaluator, EvalClass, Rank};
///
/// let hand = cards!(
///     Ace of Clubs,
///     Two of Spades,
///     Three of Diamonds,
///     Four of Diamonds,
///     Five of Clubs,
/// );
/// let eval = Evaluator::new();
/// let result = eval.evaluate(&hand).expect("couldn't evaluate hand");
/// assert!(matches!(result.class(), EvalClass::Straight { high_rank: Rank::Five }));
/// ```
///
/// [`Eval::class`]: crate::Eval::class
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvalClass {
    /// A high card, or no hand.
    HighCard {
        /// The high card rank.
        high_rank: Rank,
    },
    /// A pair, two cards of the same rank
    Pair {
        /// The ranks of the pair.
        pair: Rank,
    },
    /// Two pair. two cards of the same rank and two other cards of the same but
    /// distinct rank
    TwoPair {
        /// The ranks of the first pair.
        first_pair: Rank,
        /// The ranks of the second pair.
        second_pair: Rank,
    },
    /// Three of a kind, three cards of the same rank. Sometimes called trips.
    ThreeOfAKind {
        /// The ranks of the trips.
        trips: Rank,
    },
    /// A straight, five cards in rank order. Straights go from A2345 to TJQKA.
    Straight {
        /// The rank of the highest card in the straight.
        high_rank: Rank,
    },
    /// A flush, five cards of the same suit.
    Flush {
        /// The rank of the highest card in the flush.
        high_rank: Rank,
    },
    /// A full house, one pair and one three of a kind
    FullHouse {
        /// The rank of the trips of the full house.
        trips: Rank,
        /// The rank of the pair of the full house.
        pair: Rank,
    },
    /// Four of a kind, four cards of the same rank. Sometimes called quads.
    FourOfAKind {
        /// The rank of the quads.
        quads: Rank,
    },
    /// A straight flush, like a straight but all the cards are of the same
    /// suit.
    StraightFlush {
        /// The rank of the highest card in the straight flush.
        high_rank: Rank,
    },
}

impl fmt::Display for EvalClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::HighCard { high_rank } => write!(f, "High card, {}", high_rank.as_str_name()),
            Self::Pair { pair } => write!(f, "Pair, {}", pair.as_str_name_plural()),
            Self::TwoPair {
                first_pair: high_pair,
                second_pair: low_pair,
                ..
            } => write!(
                f,
                "Two pair, {} and {}",
                high_pair.as_str_name_plural(),
                low_pair.as_str_name_plural(),
            ),
            Self::ThreeOfAKind { trips } => {
                write!(f, "Three of a kind, {}", trips.as_str_name_plural())
            }
            Self::Straight { high_rank } => {
                write!(f, "Straight, {}-high", high_rank.as_str_name())
            }
            Self::Flush { high_rank } => write!(f, "Flush, {}-high", high_rank.as_str_name()),
            Self::FullHouse { trips, pair } => write!(
                f,
                "Full house, {} over {}",
                trips.as_str_name_plural(),
                pair.as_str_name_plural()
            ),
            Self::FourOfAKind { quads, .. } => {
                write!(f, "Four of a kind, {}", quads.as_str_name_plural())
            }
            Self::StraightFlush { high_rank, .. } => match high_rank {
                Rank::Ace => write!(f, "Royal flush"),
                high_rank => write!(f, "Straight flush, {}-high", high_rank.as_str_name()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_class_derived_ord_works() {
        let class1 = EvalClass::HighCard {
            high_rank: Rank::Ace,
        };
        let class2 = EvalClass::HighCard {
            high_rank: Rank::King,
        };
        assert!(class1 > class2);
        let class3 = EvalClass::Pair { pair: Rank::Two };
        assert!(class3 > class1);
    }
}
