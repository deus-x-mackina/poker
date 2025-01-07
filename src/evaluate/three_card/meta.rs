use crate::Rank;

use super::lookup_table::constants::*;

/// Three-card metadata isn't an `enum`; the hand type can be figured out
/// by examining the `hand_rank` field.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Meta {
    pub hand_rank: i16,
    pub card_rank: Rank,
}

impl Meta {
    pub const BEST: Self = Self {
        hand_rank: 1,
        card_rank: Rank::Ace,
    };

    pub const WORST: Self = Self {
        hand_rank: super::lookup_table::constants::WORST_HIGH_CARD,
        card_rank: Rank::Five,
    };

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
