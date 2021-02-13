use std::{cmp::Ordering, fmt, hash::Hash};

use crate::{
    card::rank::Rank,
    evaluate::{class::EvalClass, hand_rank::PokerHandRank},
};

/// Hand metadata stored in the lookup table.
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
    pub const BEST: Self = Self::StraightFlush {
        high_rank: Rank::Ace,
        hand_rank: PokerHandRank::BEST,
    };
    pub const WORST: Self = Self::HighCard {
        high_rank: Rank::Seven,
        hand_rank: PokerHandRank::WORST,
    };

    #[inline]
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

    #[inline]
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

    #[inline]
    pub const fn is_high_card(self) -> bool { matches!(self, Self::HighCard { .. }) }

    #[inline]
    pub const fn is_pair(self) -> bool { matches!(self, Self::Pair { .. }) }

    #[inline]
    pub const fn is_two_pair(self) -> bool { matches!(self, Self::TwoPair { .. }) }

    #[inline]
    pub const fn is_three_of_a_kind(self) -> bool { matches!(self, Self::ThreeOfAKind { .. }) }

    #[inline]
    pub const fn is_straight(self) -> bool { matches!(self, Self::Straight { .. }) }

    #[inline]
    pub const fn is_flush(self) -> bool { matches!(self, Self::Flush { .. }) }

    #[inline]
    pub const fn is_full_house(self) -> bool { matches!(self, Self::FullHouse { .. }) }

    #[inline]
    pub const fn is_four_of_a_kind(self) -> bool { matches!(self, Self::FourOfAKind { .. }) }

    #[inline]
    pub const fn is_straight_flush(self) -> bool { matches!(self, Self::StraightFlush { .. }) }

    #[inline]
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
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.hand_rank() == other.hand_rank() }
}

impl Eq for Meta {}

impl PartialOrd for Meta {
    #[inline]
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
    #[inline]
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
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.hand_rank().hash(state); }
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HighCard { high_rank, .. } => write!(f, "High card, {}", high_rank.as_str_name()),
            Self::Pair { pair, .. } => write!(f, "Pair, {}", pair.as_str_name_plural()),
            Self::TwoPair {
                high_pair,
                low_pair,
                ..
            } => write!(
                f,
                "Two pair, {} and {}",
                high_pair.as_str_name_plural(),
                low_pair.as_str_name_plural(),
            ),
            Self::ThreeOfAKind { trips, .. } => {
                write!(f, "Three of a kind, {}", trips.as_str_name_plural())
            }
            Self::Straight { high_rank, .. } => {
                write!(f, "Straight, {}-high", high_rank.as_str_name())
            }
            Self::Flush { high_rank, .. } => write!(f, "Flush, {}-high", high_rank.as_str_name()),
            Self::FullHouse { trips, pair, .. } => write!(
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
