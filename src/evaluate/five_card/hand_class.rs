use crate::Rank;

/// Explicit classification of five-card poker hands for pattern matching.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FiveCardHandClass {
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
    /// Hand containing exactly two pairs.
    TwoPair {
        /// The rank of the higher pair.
        high_rank: Rank,
        /// The rank of the lower pair.
        low_rank: Rank,
    },
    /// Hand containing three cards of the same rank.
    ThreeOfAKind {
        /// The rank of the three matching cards.
        rank: Rank,
    },
    /// Hand containing five consecutive ranks.
    Straight {
        /// The highest card in the straight.
        rank: Rank,
    },
    /// Hand containing five cards of the same suit.
    Flush {
        /// The highest card in the flush.
        rank: Rank,
    },
    /// Hand containing three of a kind plus a pair.
    FullHouse {
        /// The rank of the three matching cards.
        trips: Rank,
        /// The rank of the pair.
        pair: Rank,
    },
    /// Hand containing four cards of the same rank.
    FourOfAKind {
        /// The rank of the four matching cards.
        rank: Rank,
    },
    /// Hand containing five consecutive ranks of the same suit.
    StraightFlush {
        /// The highest card in the straight flush.
        rank: Rank,
    },
}
