//! Three-card poker hand evaluation implementation.
//!
//! This module provides the evaluation methods for three-card poker hands,
//! including hand type detection, ranking, and comparison. Three-card poker
//! has a simpler hand hierarchy than five-card poker, with the following
//! hand types from best to worst:
//!
//! 1. Straight flush (12 possible hands)
//! 2. Three of a kind (13 possible hands)
//! 3. Straight (12 possible hands)
//! 4. Flush (274 possible hands)
//! 5. Pair (156 possible hands)
//! 6. High card (274 possible hands)
//!
//! Total possible three-card hands: 741

use std::{cmp, fmt};

use super::{hand_class::ThreeCardHandClass, lookup_table::constants::*};
use crate::{
    Rank,
    evaluate::{eval::Eval, poker_type::ThreeCard},
};

impl Eval<ThreeCard> {
    /// The best possible three-card hand (ace-high straight flush).
    pub const BEST: Self = Self::new(1, 1 << 12);
    /// The worst possible three-card hand (three-high).
    pub const WORST: Self = Self::new(WORST_HIGH_CARD, 1 << 3);

    /// Returns `true` if this hand is better than the other hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let straight_flush: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
    /// let pair: Vec<_> = cards!("As Ac 2h").try_collect().unwrap();
    ///
    /// let sf_eval = evaluator.evaluate_three(straight_flush).unwrap();
    /// let pair_eval = evaluator.evaluate_three(pair).unwrap();
    ///
    /// assert!(sf_eval.is_better_than(pair_eval));
    /// ```
    pub const fn is_better_than(self, other: Self) -> bool { self.hand_rank() < other.hand_rank() }

    /// Returns `true` if this hand is worse than the other hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let high_card: Vec<_> = cards!("Ah Kc 2h").try_collect().unwrap();
    /// let pair: Vec<_> = cards!("As Ac 2h").try_collect().unwrap();
    ///
    /// let hc_eval = evaluator.evaluate_three(high_card).unwrap();
    /// let pair_eval = evaluator.evaluate_three(pair).unwrap();
    ///
    /// assert!(hc_eval.is_worse_than(pair_eval));
    /// ```
    pub const fn is_worse_than(self, other: Self) -> bool { self.hand_rank() > other.hand_rank() }

    /// Checks whether this evaluation would neither beat nor lose to another
    /// evaluation. Functionally equivalent to `self == other` as [`Eval`]
    /// implements [`Eq`].
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand1: Vec<_> = cards!("Ah Kc 2h").try_collect().unwrap();
    /// let hand2: Vec<_> = cards!("Ad Ks 2s").try_collect().unwrap();
    ///
    /// let eval1 = evaluator.evaluate_three(hand1).unwrap();
    /// let eval2 = evaluator.evaluate_three(hand2).unwrap();
    ///
    /// assert!(eval1.is_equal_to(eval2));
    /// assert_eq!(eval1, eval2);
    /// ```
    pub const fn is_equal_to(self, other: Self) -> bool { self.hand_rank() == other.hand_rank() }

    /// Explicitly classify this evaluation into an enum that is useful for
    /// exhaustive pattern matching. See [`ThreeCardHandClass`] for more.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards, evaluate::ThreeCardHandClass};
    ///
    /// let ev = Evaluator::new();
    /// let hand = cards!("Th Jh Qh").try_collect::<Vec<_>>().unwrap();
    /// assert!(matches!(
    ///     ev.evaluate_three(hand).unwrap().classify(),
    ///     ThreeCardHandClass::StraightFlush { rank: Rank::Queen },
    /// ));
    /// ```
    pub const fn classify(self) -> ThreeCardHandClass {
        if let Some(rank) = self.straight_flush() {
            ThreeCardHandClass::StraightFlush { rank }
        } else if let Some(rank) = self.three_of_a_kind() {
            ThreeCardHandClass::ThreeOfAKind { rank }
        } else if let Some(rank) = self.straight() {
            ThreeCardHandClass::Straight { rank }
        } else if let Some(rank) = self.flush() {
            ThreeCardHandClass::Flush { rank }
        } else if let Some(rank) = self.pair() {
            ThreeCardHandClass::Pair { rank }
        } else if let Some(rank) = self.high_card() {
            ThreeCardHandClass::HighCard { rank }
        } else {
            unreachable!();
        }
    }

    /// Returns `true` if this hand is a high card (no pairs, straights,
    /// flushes, etc.).
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kc 2d").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert!(eval.is_high_card());
    /// ```
    pub const fn is_high_card(self) -> bool {
        const BEST: i16 = WORST_PAIR + 1;
        const WORST: i16 = WORST_HIGH_CARD;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the highest card rank if this is a high card hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kc 2d").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert_eq!(eval.high_card(), Some(Rank::Ace));
    /// ```
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

    /// Returns `true` if this hand contains exactly one pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("As Ac 2d").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert!(eval.is_pair());
    /// ```
    pub const fn is_pair(self) -> bool {
        const BEST: i16 = WORST_FLUSH + 1;
        const WORST: i16 = WORST_PAIR;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the rank of the pair if this hand contains a pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("As Ac 2d").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert_eq!(eval.pair(), Some(Rank::Ace));
    /// ```
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

    /// Returns `true` if this hand is a flush (all three cards of the same
    /// suit, but not a straight flush).
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kh 2h").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert!(eval.is_flush());
    /// ```
    pub const fn is_flush(self) -> bool {
        const BEST: i16 = WORST_STRAIGHT + 1;
        const WORST: i16 = WORST_FLUSH;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the highest card rank if this hand is a flush.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kh 2h").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert_eq!(eval.flush(), Some(Rank::Ace));
    /// ```
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

    /// Returns `true` if this hand is a straight (three consecutive ranks, but
    /// not all the same suit).
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kc Qd").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert!(eval.is_straight());
    /// ```
    pub const fn is_straight(self) -> bool {
        const BEST: i16 = WORST_THREE_OF_A_KIND + 1;
        const WORST: i16 = WORST_STRAIGHT;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the highest card rank if this hand is a straight.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kc Qd").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert_eq!(eval.straight(), Some(Rank::Ace));
    /// ```
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

    /// Returns `true` if this hand contains three cards of the same rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("As Ac Ad").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert!(eval.is_three_of_a_kind());
    /// ```
    pub const fn is_three_of_a_kind(self) -> bool {
        const BEST: i16 = WORST_STRAIGHT_FLUSH + 1;
        const WORST: i16 = WORST_THREE_OF_A_KIND;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the rank of the three of a kind if this hand contains three of a
    /// kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("As Ac Ad").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert_eq!(eval.three_of_a_kind(), Some(Rank::Ace));
    /// ```
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

    /// Returns `true` if this hand is a straight flush (three consecutive
    /// ranks, all the same suit).
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert!(eval.is_straight_flush());
    /// ```
    pub const fn is_straight_flush(self) -> bool {
        const BEST: i16 = 1;
        const WORST: i16 = WORST_STRAIGHT_FLUSH;
        matches!(self.hand_rank(), BEST..=WORST)
    }

    /// Returns the highest card rank if this hand is a straight flush.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker::{Evaluator, Rank, cards};
    ///
    /// let evaluator = Evaluator::new();
    /// let hand: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
    /// let eval = evaluator.evaluate_three(hand).unwrap();
    ///
    /// assert_eq!(eval.straight_flush(), Some(Rank::Ace));
    /// ```
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
}

impl fmt::Display for Eval<ThreeCard> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(rank) = self.straight_flush() {
            f.write_fmt(format_args!("{}-high straight flush", rank.as_str_name()))
        } else if let Some(rank) = self.three_of_a_kind() {
            f.write_fmt(format_args!(
                "three of a kind, {}",
                rank.as_str_name_plural()
            ))
        } else if let Some(rank) = self.straight() {
            f.write_fmt(format_args!("{}-high straight", rank.as_str_name()))
        } else if let Some(rank) = self.flush() {
            f.write_fmt(format_args!("{}-high flush", rank.as_str_name()))
        } else if let Some(rank) = self.pair() {
            f.write_fmt(format_args!("pair, {}", rank.as_str_name_plural()))
        } else if let Some(rank) = self.high_card() {
            f.write_fmt(format_args!("high card, {}", rank.as_str_name()))
        } else {
            unreachable!()
        }
    }
}

impl cmp::Ord for Eval<ThreeCard> {
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

impl cmp::PartialOrd for Eval<ThreeCard> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> { Some(self.cmp(other)) }
}

#[cfg(test)]
mod tests {
    use super::ThreeCardHandClass;
    use crate::{Evaluator, Rank, cards};

    #[test]
    fn straight_flush_detection() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(eval.is_straight_flush());
        assert_eq!(eval.straight_flush(), Some(Rank::Ace));
        assert!(!eval.is_three_of_a_kind());
        assert!(!eval.is_straight());
        assert!(!eval.is_flush());
        assert!(!eval.is_pair());
        assert!(!eval.is_high_card());
    }

    #[test]
    fn three_of_a_kind_detection() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("As Ac Ad").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(eval.is_three_of_a_kind());
        assert_eq!(eval.three_of_a_kind(), Some(Rank::Ace));
        assert!(!eval.is_straight_flush());
        assert!(!eval.is_straight());
        assert!(!eval.is_flush());
        assert!(!eval.is_pair());
        assert!(!eval.is_high_card());
    }

    #[test]
    fn straight_detection() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Ah Kc Qd").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(eval.is_straight());
        assert_eq!(eval.straight(), Some(Rank::Ace));
        assert!(!eval.is_straight_flush());
        assert!(!eval.is_three_of_a_kind());
        assert!(!eval.is_flush());
        assert!(!eval.is_pair());
        assert!(!eval.is_high_card());
    }

    #[test]
    fn flush_detection() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Ah Kh 2h").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(eval.is_flush());
        assert_eq!(eval.flush(), Some(Rank::Ace));
        assert!(!eval.is_straight_flush());
        assert!(!eval.is_three_of_a_kind());
        assert!(!eval.is_straight());
        assert!(!eval.is_pair());
        assert!(!eval.is_high_card());
    }

    #[test]
    fn pair_detection() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("As Ac 2d").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(eval.is_pair());
        assert_eq!(eval.pair(), Some(Rank::Ace));
        assert!(!eval.is_straight_flush());
        assert!(!eval.is_three_of_a_kind());
        assert!(!eval.is_straight());
        assert!(!eval.is_flush());
        assert!(!eval.is_high_card());
    }

    #[test]
    fn high_card_detection() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Ah Kc 2d").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(eval.is_high_card());
        assert_eq!(eval.high_card(), Some(Rank::Ace));
        assert!(!eval.is_straight_flush());
        assert!(!eval.is_three_of_a_kind());
        assert!(!eval.is_straight());
        assert!(!eval.is_flush());
        assert!(!eval.is_pair());
    }

    #[test]
    fn hand_comparison() {
        let evaluator = Evaluator::new();

        let straight_flush: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
        let three_of_a_kind: Vec<_> = cards!("As Ac Ad").try_collect().unwrap();
        let straight: Vec<_> = cards!("Ah Kc Qd").try_collect().unwrap();
        let flush: Vec<_> = cards!("Ah Kh 2h").try_collect().unwrap();
        let pair: Vec<_> = cards!("As Ac 2d").try_collect().unwrap();
        let high_card: Vec<_> = cards!("Ah Kc 2d").try_collect().unwrap();

        let sf_eval = evaluator.evaluate_three(straight_flush).unwrap();
        let trip_eval = evaluator.evaluate_three(three_of_a_kind).unwrap();
        let str_eval = evaluator.evaluate_three(straight).unwrap();
        let flush_eval = evaluator.evaluate_three(flush).unwrap();
        let pair_eval = evaluator.evaluate_three(pair).unwrap();
        let hc_eval = evaluator.evaluate_three(high_card).unwrap();

        // Test all comparisons
        assert!(sf_eval.is_better_than(trip_eval));
        assert!(trip_eval.is_better_than(str_eval));
        assert!(str_eval.is_better_than(flush_eval));
        assert!(flush_eval.is_better_than(pair_eval));
        assert!(pair_eval.is_better_than(hc_eval));

        // Test reverse comparisons
        assert!(hc_eval.is_worse_than(pair_eval));
        assert!(pair_eval.is_worse_than(flush_eval));
        assert!(flush_eval.is_worse_than(str_eval));
        assert!(str_eval.is_worse_than(trip_eval));
        assert!(trip_eval.is_worse_than(sf_eval));
    }

    #[test]
    fn display_format() {
        let evaluator = Evaluator::new();

        let straight_flush: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
        let sf_eval = evaluator.evaluate_three(straight_flush).unwrap();
        assert_eq!(format!("{}", sf_eval), "ace-high straight flush");

        let three_of_a_kind: Vec<_> = cards!("As Ac Ad").try_collect().unwrap();
        let trip_eval = evaluator.evaluate_three(three_of_a_kind).unwrap();
        assert_eq!(format!("{}", trip_eval), "three of a kind, aces");

        let straight: Vec<_> = cards!("Ah Kc Qd").try_collect().unwrap();
        let str_eval = evaluator.evaluate_three(straight).unwrap();
        assert_eq!(format!("{}", str_eval), "ace-high straight");

        let flush: Vec<_> = cards!("Ah Kh 2h").try_collect().unwrap();
        let flush_eval = evaluator.evaluate_three(flush).unwrap();
        assert_eq!(format!("{}", flush_eval), "ace-high flush");

        let pair: Vec<_> = cards!("As Ac 2d").try_collect().unwrap();
        let pair_eval = evaluator.evaluate_three(pair).unwrap();
        assert_eq!(format!("{}", pair_eval), "pair, aces");

        let high_card: Vec<_> = cards!("Ah Kc 2d").try_collect().unwrap();
        let hc_eval = evaluator.evaluate_three(high_card).unwrap();
        assert_eq!(format!("{}", hc_eval), "high card, ace");
    }

    #[test]
    fn best_and_worst_constants() {
        let evaluator = Evaluator::new();

        // Best possible hand: A-K-Q straight flush
        let best_hand: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
        let best_eval = evaluator.evaluate_three(best_hand).unwrap();

        // Worst possible hand: 4-3-2 high card
        let worst_hand: Vec<_> = cards!("4h 3c 2d").try_collect().unwrap();
        let worst_eval = evaluator.evaluate_three(worst_hand).unwrap();

        assert!(best_eval.is_better_than(worst_eval));
        assert!(worst_eval.is_worse_than(best_eval));
    }

    #[test]
    fn low_straight_detection() {
        let evaluator = Evaluator::new();

        // Test A-2-3 straight (wheel)
        let wheel: Vec<_> = cards!("Ah 2c 3d").try_collect().unwrap();
        let wheel_eval = evaluator.evaluate_three(wheel).unwrap();

        assert!(wheel_eval.is_straight());
        assert_eq!(wheel_eval.straight(), Some(Rank::Three));
    }

    #[test]
    fn classify_straight_flush() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Th Jh Qh").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(matches!(
            eval.classify(),
            ThreeCardHandClass::StraightFlush { rank: Rank::Queen }
        ));

        // Test ace-high straight flush
        let ace_sf: Vec<_> = cards!("Ah Kh Qh").try_collect().unwrap();
        let ace_eval = evaluator.evaluate_three(ace_sf).unwrap();
        assert!(matches!(
            ace_eval.classify(),
            ThreeCardHandClass::StraightFlush { rank: Rank::Ace }
        ));
    }

    #[test]
    fn classify_three_of_a_kind() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("As Ac Ad").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(matches!(
            eval.classify(),
            ThreeCardHandClass::ThreeOfAKind { rank: Rank::Ace }
        ));
    }

    #[test]
    fn classify_straight() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Ah Kc Qd").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(matches!(
            eval.classify(),
            ThreeCardHandClass::Straight { rank: Rank::Ace }
        ));

        // Test wheel straight (A-2-3)
        let wheel: Vec<_> = cards!("Ah 2c 3d").try_collect().unwrap();
        let wheel_eval = evaluator.evaluate_three(wheel).unwrap();
        assert!(matches!(
            wheel_eval.classify(),
            ThreeCardHandClass::Straight { rank: Rank::Three }
        ));
    }

    #[test]
    fn classify_flush() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Ah Kh 2h").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(matches!(
            eval.classify(),
            ThreeCardHandClass::Flush { rank: Rank::Ace }
        ));
    }

    #[test]
    fn classify_pair() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("As Ac 2d").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(matches!(
            eval.classify(),
            ThreeCardHandClass::Pair { rank: Rank::Ace }
        ));
    }

    #[test]
    fn classify_high_card() {
        let evaluator = Evaluator::new();
        let hand: Vec<_> = cards!("Ah Kc 2d").try_collect().unwrap();
        let eval = evaluator.evaluate_three(hand).unwrap();

        assert!(matches!(
            eval.classify(),
            ThreeCardHandClass::HighCard { rank: Rank::Ace }
        ));
    }

    #[test]
    fn classify_exhaustive_pattern_matching() {
        let evaluator = Evaluator::new();

        let hands = vec![
            (
                cards!("Ah Kh Qh").try_collect::<Vec<_>>().unwrap(),
                "straight flush",
            ),
            (
                cards!("As Ac Ad").try_collect::<Vec<_>>().unwrap(),
                "three of a kind",
            ),
            (
                cards!("Ah Kc Qd").try_collect::<Vec<_>>().unwrap(),
                "straight",
            ),
            (cards!("Ah Kh 2h").try_collect::<Vec<_>>().unwrap(), "flush"),
            (cards!("As Ac 2d").try_collect::<Vec<_>>().unwrap(), "pair"),
            (
                cards!("Ah Kc 2d").try_collect::<Vec<_>>().unwrap(),
                "high card",
            ),
        ];

        for (hand, expected) in hands {
            let eval = evaluator.evaluate_three(hand).unwrap();
            let description = match eval.classify() {
                ThreeCardHandClass::StraightFlush { .. } => "straight flush",
                ThreeCardHandClass::ThreeOfAKind { .. } => "three of a kind",
                ThreeCardHandClass::Straight { .. } => "straight",
                ThreeCardHandClass::Flush { .. } => "flush",
                ThreeCardHandClass::Pair { .. } => "pair",
                ThreeCardHandClass::HighCard { .. } => "high card",
            };
            assert_eq!(description, expected);
        }
    }
}
