use crate::Rank;

/// Explicit classification of three-card poker hands for pattern matching.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ThreeCardHandClass {
    /// High card hand with no pairs, straights, or flushes.
    HighCard {
        /// The highest card in the hand.
        rank: Rank,
    },
    /// Hand containing exactly one pair.
    Pair {
        /// The rank of the pair.
        rank: Rank,
    },
    /// Hand containing three cards of the same suit (but not a straight).
    Flush {
        /// The highest card in the flush.
        rank: Rank,
    },
    /// Hand containing three consecutive ranks (but not all the same suit).
    Straight {
        /// The highest card in the straight.
        rank: Rank,
    },
    /// Hand containing three cards of the same rank.
    ThreeOfAKind {
        /// The rank of the three matching cards.
        rank: Rank,
    },
    /// Hand containing three consecutive ranks of the same suit.
    StraightFlush {
        /// The highest card in the straight flush.
        rank: Rank,
    },
}
