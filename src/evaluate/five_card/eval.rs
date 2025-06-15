use std::{cmp, fmt};

use super::{hand_class::FiveCardHandClass, lookup_table::constants::*};
use crate::{
    Rank,
    evaluate::{eval::Eval, poker_type::FiveCard},
};

impl Eval<FiveCard> {
    /// The best possible five-card hand evaluation.
    pub const BEST: Self = Self::new(1, 1 << 12);
    /// The worst possible five-card hand evaluation.
    pub const WORST: Self = Self::new(7462, 1 << 5);

    /// Checks whether this evaluation would beat another evaluation.
    pub const fn is_better_than(self, other: Self) -> bool { self.hand_rank() < other.hand_rank() }

    /// Chechks whether this evaluation would lose to another evaluation.
    pub const fn is_worse_than(self, other: Self) -> bool { self.hand_rank() > other.hand_rank() }

    /// Checks whether this evaluation would neither beat nor lose to another
    /// evaluation. Functionally equivalent to `self == other` as [`Eval`]
    /// implements [`Eq`].
    pub const fn is_equal_to(self, other: Self) -> bool { self.hand_rank() == other.hand_rank() }

    /// Explicity classify this evaluation into an enum that is useful for
    /// exhaustive pattern matching. See [`FiveCardHandClass`] for more.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards, evaluate::FiveCardHandClass};
    ///
    /// let ev = Evaluator::new();
    /// let hand = cards!("Th Jh Qh Kh Ah").try_collect::<Vec<_>>().unwrap();
    /// assert!(matches!(
    ///     ev.evaluate_five(hand).unwrap().classify(),
    ///     FiveCardHandClass::StraightFlush { rank: Rank::Ace },
    /// ));
    /// ```
    pub const fn classify(self) -> FiveCardHandClass {
        if let Some(rank) = self.straight_flush() {
            FiveCardHandClass::StraightFlush { rank }
        } else if let Some(rank) = self.four_of_a_kind() {
            FiveCardHandClass::FourOfAKind { rank }
        } else if let Some((trips, pair)) = self.full_house() {
            FiveCardHandClass::FullHouse { trips, pair }
        } else if let Some(rank) = self.flush() {
            FiveCardHandClass::Flush { rank }
        } else if let Some(rank) = self.straight() {
            FiveCardHandClass::Straight { rank }
        } else if let Some(rank) = self.three_of_a_kind() {
            FiveCardHandClass::ThreeOfAKind { rank }
        } else if let Some((high_rank, low_rank)) = self.two_pair() {
            FiveCardHandClass::TwoPair {
                high_rank,
                low_rank,
            }
        } else if let Some(rank) = self.pair() {
            FiveCardHandClass::Pair { rank }
        } else if let Some(rank) = self.high_card() {
            FiveCardHandClass::HighCard { rank }
        } else {
            unreachable!();
        }
    }

    /// Returns true if this hand is a high card (no pairs, straights, or
    /// flushes).
    pub const fn is_high_card(self) -> bool {
        const BEST: i16 = WORST_PAIR + 1;
        const WORST: i16 = WORST_HIGH_CARD;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the high card rank if this hand is a high card, None otherwise.
    pub const fn high_card(self) -> Option<Rank> {
        if self.is_high_card() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            Some(Rank::ALL_VARIANTS[i])
        } else {
            None
        }
    }

    /// Returns true if this hand contains exactly one pair.
    pub const fn is_pair(self) -> bool {
        const BEST: i16 = WORST_TWO_PAIR + 1;
        const WORST: i16 = WORST_PAIR;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the rank of the pair if this hand contains exactly one pair,
    /// None otherwise.
    pub const fn pair(self) -> Option<Rank> {
        if self.is_pair() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            Some(Rank::ALL_VARIANTS[i])
        } else {
            None
        }
    }

    /// Returns true if this hand contains exactly two pairs.
    pub const fn is_two_pair(self) -> bool {
        const BEST: i16 = WORST_THREE_OF_A_KIND + 1;
        const WORST: i16 = WORST_TWO_PAIR;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the ranks of both pairs if this hand contains two pairs, None
    /// otherwise. The first rank is the higher pair, the second is the
    /// lower pair.
    pub const fn two_pair(self) -> Option<(Rank, Rank)> {
        if self.is_two_pair() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            let mut j = i + 1;
            while flags & (1 << j) == 0 {
                j += 1;
            }
            Some((Rank::ALL_VARIANTS[j], Rank::ALL_VARIANTS[i]))
        } else {
            None
        }
    }

    /// Returns true if this hand contains three cards of the same rank.
    pub const fn is_three_of_a_kind(self) -> bool {
        const BEST: i16 = WORST_STRAIGHT + 1;
        const WORST: i16 = WORST_THREE_OF_A_KIND;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the rank of the three-of-a-kind if this hand contains one, None
    /// otherwise.
    pub const fn three_of_a_kind(self) -> Option<Rank> {
        if self.is_three_of_a_kind() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            Some(Rank::ALL_VARIANTS[i])
        } else {
            None
        }
    }

    /// Returns true if this hand is a straight (five consecutive ranks).
    pub const fn is_straight(self) -> bool {
        const BEST: i16 = WORST_FLUSH + 1;
        const WORST: i16 = WORST_STRAIGHT;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the high card of the straight if this hand is a straight, None
    /// otherwise.
    pub const fn straight(self) -> Option<Rank> {
        if self.is_straight() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            Some(Rank::ALL_VARIANTS[i])
        } else {
            None
        }
    }

    /// Returns true if this hand is a flush (five cards of the same suit).
    pub const fn is_flush(self) -> bool {
        const BEST: i16 = WORST_FULL_HOUSE + 1;
        const WORST: i16 = WORST_FLUSH;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the high card of the flush if this hand is a flush, None
    /// otherwise.
    pub const fn flush(self) -> Option<Rank> {
        if self.is_flush() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            Some(Rank::ALL_VARIANTS[i])
        } else {
            None
        }
    }

    /// Returns true if this hand is a full house (three of a kind plus a pair).
    pub const fn is_full_house(self) -> bool {
        const BEST: i16 = WORST_FOUR_OF_A_KIND + 1;
        const WORST: i16 = WORST_FULL_HOUSE;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the ranks of the full house if this hand is a full house, None
    /// otherwise. The first rank is the three-of-a-kind, the second is the
    /// pair.
    pub const fn full_house(self) -> Option<(Rank, Rank)> {
        if self.is_full_house() {
            let flags = self.rank_flags();
            let rev = flags.leading_ones() == 1;
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            let mut j = i + 1;
            while flags & (1 << j) == 0 {
                j += 1;
            }
            if rev {
                Some((Rank::ALL_VARIANTS[i], Rank::ALL_VARIANTS[j]))
            } else {
                Some((Rank::ALL_VARIANTS[j], Rank::ALL_VARIANTS[i]))
            }
        } else {
            None
        }
    }

    /// Returns true if this hand contains four cards of the same rank.
    pub const fn is_four_of_a_kind(self) -> bool {
        const BEST: i16 = WORST_STRAIGHT_FLUSH + 1;
        const WORST: i16 = WORST_FOUR_OF_A_KIND;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the rank of the four-of-a-kind if this hand contains one, None
    /// otherwise.
    pub const fn four_of_a_kind(self) -> Option<Rank> {
        if self.is_four_of_a_kind() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            Some(Rank::ALL_VARIANTS[i])
        } else {
            None
        }
    }

    /// Returns true if this hand is a straight flush (five consecutive ranks of
    /// the same suit).
    pub const fn is_straight_flush(self) -> bool {
        const BEST: i16 = 1;
        const WORST: i16 = WORST_STRAIGHT_FLUSH;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the high card of the straight flush if this hand is a straight
    /// flush, None otherwise.
    pub const fn straight_flush(self) -> Option<Rank> {
        if self.is_straight_flush() {
            let flags = self.rank_flags();
            let mut i = 0;
            while flags & (1 << i) == 0 {
                i += 1;
            }
            Some(Rank::ALL_VARIANTS[i])
        } else {
            None
        }
    }

    /// Returns true if this hand is a royal flush (A-K-Q-J-10 of the same
    /// suit).
    pub const fn is_royal_flush(self) -> bool { self.hand_rank() == 1 }
}

impl fmt::Display for Eval<FiveCard> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_royal_flush() {
            f.write_str("royal flush")
        } else if let Some(rank) = self.straight_flush() {
            f.write_fmt(format_args!("{}-high straight flush", rank.as_str_name()))
        } else if let Some(rank) = self.four_of_a_kind() {
            f.write_fmt(format_args!(
                "four of a kind, {}",
                rank.as_str_name_plural()
            ))
        } else if let Some((high, low)) = self.full_house() {
            f.write_fmt(format_args!(
                "full house, {} over {}",
                high.as_str_name_plural(),
                low.as_str_name_plural()
            ))
        } else if let Some(rank) = self.flush() {
            f.write_fmt(format_args!("{}-high flush", rank.as_str_name()))
        } else if let Some(rank) = self.straight() {
            f.write_fmt(format_args!("{}-high straight", rank.as_str_name()))
        } else if let Some(rank) = self.three_of_a_kind() {
            f.write_fmt(format_args!(
                "three of a kind, {}",
                rank.as_str_name_plural()
            ))
        } else if let Some((high, low)) = self.two_pair() {
            f.write_fmt(format_args!(
                "two pair, {} and {}",
                high.as_str_name_plural(),
                low.as_str_name_plural()
            ))
        } else if let Some(rank) = self.pair() {
            f.write_fmt(format_args!("pair, {}", rank.as_str_name_plural()))
        } else if let Some(rank) = self.high_card() {
            f.write_fmt(format_args!("high card, {}", rank.as_str_name()))
        } else {
            unreachable!()
        }
    }
}

impl cmp::Ord for Eval<FiveCard> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let s = self.hand_rank();
        let o = other.hand_rank();
        if s < o {
            cmp::Ordering::Greater
        } else if s > o {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }
    }
}

impl cmp::PartialOrd for Eval<FiveCard> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> { Some(self.cmp(other)) }
}

#[cfg(test)]
mod tests {
    use crate::evaluate::five_card::lookup_table::LookupTable;

    #[test]
    fn all_lookup_table_entries_classify() {
        let LookupTable {
            flush_lookup,
            unsuited_lookup,
        } = LookupTable::new();

        // we just want to ensure `unreachable!()` doesn't run.
        flush_lookup.values().for_each(|&eval| {
            eval.classify();
        });

        unsuited_lookup.values().for_each(|&eval| {
            eval.classify();
        });
    }
}
