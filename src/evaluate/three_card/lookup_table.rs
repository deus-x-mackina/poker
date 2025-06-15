//! Lookup table generation for three-card poker hand evaluation.
//!
//! This module contains the logic for generating the lookup tables used
//! in three-card poker hand evaluation. The tables map the product of
//! prime numbers (representing cards) to their corresponding hand rankings.
//!
//! # Three-Card Hand Distribution
//!
//! The total number of unique three-card hands is 741, distributed as follows:
//!
//! | Hand Type        | Count |
//! |------------------|-------|
//! | Straight flush   |    12 |
//! | Three of a kind  |    13 |
//! | Straight         |    12 |
//! | Flush            |   274 |
//! | Pair             |   156 |
//! | High card        |   274 |
//! | **Total**        | **741** |

use constants::{
    STRAIGHTS, WORST_FLUSH, WORST_PAIR, WORST_STRAIGHT, WORST_STRAIGHT_FLUSH, WORST_THREE_OF_A_KIND,
};
use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::{
    constants::{INT_RANKS, PRIMES},
    evaluate::{
        eval::Eval,
        poker_type::ThreeCard,
        utils::{self, BitSequence},
    },
};

/// Lookup table for three-card poker hand evaluation.
///
/// This structure contains two hash maps:
/// - `flush_lookup`: For hands where all three cards have the same suit
/// - `unsuited_lookup`: For hands where cards have mixed suits
///
/// The keys are products of prime numbers representing the cards,
/// and the values are the corresponding hand evaluations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupTable {
    /// Lookup table for flush hands (including straight flushes)
    pub flush_lookup: FxHashMap<i32, Eval<ThreeCard>>,
    /// Lookup table for non-flush hands
    pub unsuited_lookup: FxHashMap<i32, Eval<ThreeCard>>,
}

impl LookupTable {
    /// Creates a new lookup table for three-card poker evaluation.
    ///
    /// This method generates all possible three-card combinations and
    /// assigns them appropriate hand rankings. The construction process
    /// involves:
    /// 1. Generating flush hands (including straight flushes)
    /// 2. Generating non-flush hands (straights, pairs, high cards, three of a
    ///    kind)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use poker::evaluate::three_card::lookup_table::LookupTable;
    /// let table = LookupTable::new();
    /// // Table is now ready for hand evaluation
    /// ```
    pub fn new() -> Self {
        let mut lookup_table = Self {
            flush_lookup: FxHashMap::with_capacity_and_hasher(286, FxBuildHasher),
            unsuited_lookup: FxHashMap::with_capacity_and_hasher(455, FxBuildHasher),
        };
        lookup_table.flushes();
        lookup_table.multiples();
        lookup_table
    }

    /// Generates flush and straight flush hands.
    ///
    /// This method populates the flush_lookup table with all possible
    /// flush combinations, including straight flushes.
    fn flushes(&mut self) {
        const NOT_STRAIGHTS: [i16; 274] = constants::not_straights();

        let mut rank_suited: i16 = 1;
        let mut rank_unsuited: i16 = WORST_THREE_OF_A_KIND + 1;

        let mut rank_flags;
        let mut prime_product;

        for straight in STRAIGHTS {
            prime_product = utils::prime_product_from_rank_bits(straight);
            rank_flags = utils::high_rank_from_rank_bits_three(straight);
            self.flush_lookup
                .insert(prime_product, Eval::new(rank_suited, rank_flags));
            self.unsuited_lookup
                .insert(prime_product, Eval::new(rank_unsuited, rank_flags));
            rank_suited = rank_suited.wrapping_add(1);
            rank_unsuited = rank_unsuited.wrapping_add(1);
        }

        rank_suited = WORST_STRAIGHT + 1;
        rank_unsuited = WORST_PAIR + 1;

        for bits in NOT_STRAIGHTS.into_iter().rev() {
            prime_product = utils::prime_product_from_rank_bits(bits);
            rank_flags = utils::high_rank_from_rank_bits_three(bits);
            self.flush_lookup
                .insert(prime_product, Eval::new(rank_suited, rank_flags));
            self.unsuited_lookup
                .insert(prime_product, Eval::new(rank_unsuited, rank_flags));
            rank_suited = rank_suited.wrapping_add(1);
            rank_unsuited = rank_unsuited.wrapping_add(1);
        }
    }

    /// Generates non-flush hands including straights, pairs, three of a kind,
    /// and high cards.
    ///
    /// This method populates the unsuited_lookup table with all possible
    /// non-flush combinations.
    fn multiples(&mut self) {
        let mut product;

        // Three of a kind
        let mut hand_rank = WORST_STRAIGHT_FLUSH + 1;

        for trips in INT_RANKS.rev() {
            product = PRIMES[trips as usize].wrapping_pow(3);
            self.unsuited_lookup.insert(
                product,
                Eval::new(hand_rank, 1i16.wrapping_shl(trips as u32)),
            );
            hand_rank = hand_rank.wrapping_add(1);
        }

        // Pair
        hand_rank = WORST_FLUSH + 1;

        for pair in INT_RANKS.rev() {
            let kickers = INT_RANKS.rev().filter(|&i| i != pair);
            for kicker in kickers {
                product = PRIMES[pair as usize]
                    .wrapping_pow(2)
                    .wrapping_mul(PRIMES[kicker as usize]);
                self.unsuited_lookup.insert(
                    product,
                    Eval::new(hand_rank, 1i16.wrapping_shl(pair as u32)),
                );
                hand_rank = hand_rank.wrapping_add(1);
            }
        }
    }
}

impl Default for LookupTable {
    fn default() -> Self { Self::new() }
}

pub mod constants {
    use super::*;

    pub const WORST_STRAIGHT_FLUSH: i16 = 12;
    pub const WORST_THREE_OF_A_KIND: i16 = 25;
    pub const WORST_STRAIGHT: i16 = 37;
    pub const WORST_FLUSH: i16 = 311;
    pub const WORST_PAIR: i16 = 467;
    pub const WORST_HIGH_CARD: i16 = 741;

    pub const STRAIGHTS: [i16; 12] = [
        0b1_1100_0000_0000,
        0b0_1110_0000_0000,
        0b0_0111_0000_0000,
        0b0_0011_1000_0000,
        0b0_0001_1100_0000,
        0b0_0000_1110_0000,
        0b0_0000_0111_0000,
        0b0_0000_0011_1000,
        0b0_0000_0001_1100,
        0b0_0000_0000_1110,
        0b0_0000_0000_0111,
        0b1_0000_0000_0011,
    ];

    pub const fn not_straights() -> [i16; 274] {
        let mut not_straights = [0; 274];
        let mut generator = BitSequence::new(0b111);
        let mut i = 0;
        let mut cur = 0;
        while i < 285 {
            let bits = generator.get_next();
            let mut is_straight = false;
            let mut j = 0;
            while j < STRAIGHTS.len() {
                let straight = STRAIGHTS[j];
                if bits ^ straight == 0 {
                    is_straight = true;
                    break;
                }
                j += 1;
            }
            if !is_straight {
                not_straights[cur] = bits;
                cur += 1;
            }
            i += 1;
        }
        // if cur != 274 { panic!() }
        not_straights
    }
}

#[cfg(test)]
mod tests {
    use super::{constants::*, *};

    #[test]
    fn lookup_table_creation() {
        let table = LookupTable::new();

        // Check that we have the expected number of entries
        // Flush hands: 274 (non-straight) + 12 (straight flush) = 286
        assert_eq!(table.flush_lookup.len(), 286);

        // Non-flush hands: 13 (three of a kind) + 12 (straight) + 156 (pair) + 274
        // (high card) = 455
        assert_eq!(table.unsuited_lookup.len(), 455);
    }

    #[test]
    fn all_lookup_table_entries_have_valid_rankings() {
        let table = LookupTable::new();

        // Check that all flush lookups have valid hand rankings
        for &eval in table.flush_lookup.values() {
            let rank = eval.hand_rank();
            assert!((1..=WORST_FLUSH).contains(&rank));
        }

        // Check that all unsuited lookups have valid hand rankings
        for &eval in table.unsuited_lookup.values() {
            let rank = eval.hand_rank();
            assert!(rank > WORST_STRAIGHT_FLUSH && rank <= WORST_HIGH_CARD);
        }
    }

    #[test]
    fn straight_constants() {
        // Verify we have the correct number of straights
        assert_eq!(STRAIGHTS.len(), 12);

        // Test a few known straights
        assert_eq!(STRAIGHTS[0], 0b1_1100_0000_0000); // A-K-Q
        assert_eq!(STRAIGHTS[11], 0b1_0000_0000_0011); // A-2-3 (wheel)
    }

    #[test]
    fn not_straights_generation() {
        let not_straights = constants::not_straights();

        // Should have exactly 274 non-straight flush combinations
        assert_eq!(not_straights.len(), 274);
    }

    #[test]
    fn default_implementation() {
        let table1 = LookupTable::new();
        let table2 = LookupTable::default();

        assert_eq!(table1, table2);
    }
}
