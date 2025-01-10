use std::marker::PhantomData;

use super::lookup_table::constants::*;
use crate::evaluate::poker_type::{PokerType, ThreeCard};

/// Three-card metadata isn't an `enum`; the hand type can be figured out
/// by examining the `hand_rank` field.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Meta<P> {
    /// The rank of the poker hand.
    hand_rank: i16,
    /// Extra info about cards in the hand (e.g., pair of `Jacks` vs `Tens`).
    /// Can encode information for 1 or 2 ranks, as bitfields:
    /// 1 << 0 = 2
    /// 1 << 12 = Ace
    rank_flags: i16,
    /// Marks metadata as a three- or five-card poker hand.
    _poker_type: PhantomData<P>,
}

impl<P: PokerType> Meta<P> {
    pub const fn new(hand_rank: i16, rank_flags: i16) -> Self {
        Self {
            hand_rank,
            rank_flags,
            _poker_type: PhantomData,
        }
    }
}

impl Meta<ThreeCard> {
    pub const BEST: Self = Self::new(1, 1 << 12);
    pub const WORST: Self = Self::new(WORST_HIGH_CARD, 1 << 3);

    pub const fn is_high_card(self) -> bool {
        const BEST: i16 = WORST_PAIR + 1;
        const WORST: i16 = WORST_HIGH_CARD;
        matches!(self.hand_rank, BEST..=WORST)
    }

    pub const fn is_pair(self) -> bool {
        const BEST: i16 = WORST_FLUSH + 1;
        const WORST: i16 = WORST_PAIR;
        matches!(self.hand_rank, BEST..=WORST)
    }

    pub const fn is_flush(self) -> bool {
        const BEST: i16 = WORST_STRAIGHT + 1;
        const WORST: i16 = WORST_FLUSH;
        matches!(self.hand_rank, BEST..=WORST)
    }

    pub const fn is_straight(self) -> bool {
        const BEST: i16 = WORST_THREE_OF_A_KIND + 1;
        const WORST: i16 = WORST_STRAIGHT;
        matches!(self.hand_rank, BEST..=WORST)
    }

    pub const fn is_three_of_a_kind(self) -> bool {
        const BEST: i16 = WORST_STRAIGHT_FLUSH + 1;
        const WORST: i16 = WORST_THREE_OF_A_KIND;
        matches!(self.hand_rank, BEST..=WORST)
    }

    pub const fn is_straight_flush(self) -> bool {
        const BEST: i16 = 1;
        const WORST: i16 = WORST_STRAIGHT_FLUSH;
        matches!(self.hand_rank, BEST..=WORST)
    }
}
