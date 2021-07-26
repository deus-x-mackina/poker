use std::{cmp::Ordering, fmt, hash::Hash};

use crate::{
    card::rank::Rank,
    evaluate::{class::EvalClass, hand_rank::PokerHandRank},
};

/// Hand metadata stored in the lookup table. It contains slightly more
/// information than a `EvalClass`, and is not meant to be public/constructed by
/// hand.
#[derive(Debug, Copy, Clone)]
pub enum Meta {
    HighCard {
        hand_rank: PokerHandRank,
        high_rank: Rank,
    },
    Pair {
        hand_rank: PokerHandRank,
        pair: Rank,
    },
    TwoPair {
        hand_rank: PokerHandRank,
        high_pair: Rank,
        low_pair: Rank,
    },
    ThreeOfAKind {
        hand_rank: PokerHandRank,
        trips: Rank,
    },
    Straight {
        hand_rank: PokerHandRank,
        high_rank: Rank,
    },
    Flush {
        hand_rank: PokerHandRank,
        high_rank: Rank,
    },
    FullHouse {
        hand_rank: PokerHandRank,
        trips: Rank,
        pair: Rank,
    },
    FourOfAKind {
        hand_rank: PokerHandRank,
        quads: Rank,
    },
    StraightFlush {
        hand_rank: PokerHandRank,
        high_rank: Rank,
    },
}

impl Meta {
    /// The best possible hand
    pub const BEST: Self = Self::StraightFlush {
        high_rank: Rank::Ace,
        hand_rank: PokerHandRank::BEST,
    };
    /// The worst possible hand
    pub const WORST: Self = Self::HighCard {
        high_rank: Rank::Seven,
        hand_rank: PokerHandRank::WORST,
    };

    pub(crate) const fn hand_rank(self) -> PokerHandRank {
        match self {
            // Is there a more elegant way to do this?
            Self::HighCard { hand_rank, .. }
            | Self::Pair { hand_rank, .. }
            | Self::TwoPair { hand_rank, .. }
            | Self::ThreeOfAKind { hand_rank, .. }
            | Self::Straight { hand_rank, .. }
            | Self::Flush { hand_rank, .. }
            | Self::FullHouse { hand_rank, .. }
            | Self::FourOfAKind { hand_rank, .. }
            | Self::StraightFlush { hand_rank, .. } => hand_rank,
        }
    }

    pub const fn class(self) -> EvalClass {
        match self {
            Self::HighCard { high_rank, .. } => EvalClass::HighCard { high_rank },
            Self::Pair { pair, .. } => EvalClass::Pair { pair },
            Self::TwoPair {
                high_pair,
                low_pair,
                ..
            } => EvalClass::TwoPair {
                first_pair: high_pair,
                second_pair: low_pair,
            },
            Self::ThreeOfAKind { trips, .. } => EvalClass::ThreeOfAKind { trips },
            Self::Straight { high_rank, .. } => EvalClass::Straight { high_rank },
            Self::Flush { high_rank, .. } => EvalClass::Flush { high_rank },
            Self::FullHouse { trips, pair, .. } => EvalClass::FullHouse { trips, pair },
            Self::FourOfAKind { quads, .. } => EvalClass::FourOfAKind { quads },
            Self::StraightFlush { high_rank, .. } => EvalClass::StraightFlush { high_rank },
        }
    }

    pub const fn is_high_card(self) -> bool { matches!(self, Self::HighCard { .. }) }

    pub const fn is_pair(self) -> bool { matches!(self, Self::Pair { .. }) }

    pub const fn is_two_pair(self) -> bool { matches!(self, Self::TwoPair { .. }) }

    pub const fn is_three_of_a_kind(self) -> bool { matches!(self, Self::ThreeOfAKind { .. }) }

    pub const fn is_straight(self) -> bool { matches!(self, Self::Straight { .. }) }

    pub const fn is_flush(self) -> bool { matches!(self, Self::Flush { .. }) }

    pub const fn is_full_house(self) -> bool { matches!(self, Self::FullHouse { .. }) }

    pub const fn is_four_of_a_kind(self) -> bool { matches!(self, Self::FourOfAKind { .. }) }

    pub const fn is_straight_flush(self) -> bool { matches!(self, Self::StraightFlush { .. }) }

    pub const fn is_royal_flush(self) -> bool {
        matches!(
            self,
            Self::StraightFlush {
                hand_rank: PokerHandRank::BEST,
                ..
            }
        )
    }
}

impl PartialEq for Meta {
    fn eq(&self, other: &Self) -> bool { self.hand_rank() == other.hand_rank() }
}

impl Eq for Meta {}

impl PartialOrd for Meta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_rank_self = self.hand_rank();
        let hand_rank_other = other.hand_rank();
        if hand_rank_self.is_better_than(hand_rank_other) {
            Some(Ordering::Greater)
        } else if hand_rank_self == hand_rank_other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for Meta {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_rank_self = self.hand_rank();
        let hand_rank_other = other.hand_rank();
        if hand_rank_self.is_better_than(hand_rank_other) {
            Ordering::Greater
        } else if hand_rank_self == hand_rank_other {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl Hash for Meta {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.hand_rank().hash(state); }
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.class().fmt(f) }
}
