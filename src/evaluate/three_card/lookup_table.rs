use crate::{constants::{INT_RANKS, PRIMES}, evaluate::utils::{self, BitSequence}, Rank};

use super::meta::Meta;

use constants::{STRAIGHTS, WORST_FLUSH, WORST_PAIR, WORST_STRAIGHT, WORST_STRAIGHT_FLUSH, WORST_THREE_OF_A_KIND};
use rustc_hash::{FxBuildHasher, FxHashMap};

/// Three card hands:
/// 
/// 1. Straight flush   12
/// 2. Three of a kind  13
/// 3. Straight         12
/// 4. Flush           274
/// 5. Pair            156
/// 6. High card     + 274
/// ----------------------
/// Total            = 741

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupTable {
    pub flush_lookup: FxHashMap<i32, Meta>,
    pub unsuited_lookup: FxHashMap<i32, Meta>,
}

impl LookupTable {
    pub fn new() -> Self {
        let mut lookup_table = Self {
            flush_lookup: FxHashMap::with_capacity_and_hasher(286, FxBuildHasher),
            unsuited_lookup: FxHashMap::with_capacity_and_hasher(455, FxBuildHasher),
        };
        lookup_table.flushes();
        lookup_table.multiples();
        lookup_table
    }

    fn flushes(&mut self) {
        let mut not_straights = Vec::with_capacity(276);
        let mut gen = BitSequence::new(0b111);
        for _ in 0..285 {
            let bits = gen.get_next();
            let mut is_straight = false;
            for straight in STRAIGHTS {
                if bits ^ straight == 0 {
                    is_straight = true;
                    break;
                }
            }
            if !is_straight {
                not_straights.push(bits);
            }
        }

        let mut rank_suited = 1;
        let mut rank_unsuited = WORST_THREE_OF_A_KIND + 1;

        let mut high_rank;
        let mut prime_product;

        for straight in STRAIGHTS {
            prime_product = utils::prime_product_from_rank_bits(straight);
            high_rank = utils::high_rank_from_rank_bits(straight);
            self.flush_lookup.insert(prime_product, Meta {
                hand_rank: rank_suited,
                card_rank: high_rank,
            });
            self.unsuited_lookup.insert(prime_product, Meta {
                hand_rank: rank_unsuited,
                card_rank: high_rank,
            });
            rank_suited = rank_suited.wrapping_add(1);
            rank_unsuited = rank_unsuited.wrapping_add(1);
        }

        rank_suited = WORST_STRAIGHT + 1;
        rank_unsuited = WORST_PAIR + 1;

        for bits in not_straights.into_iter().rev() {
            prime_product = utils::prime_product_from_rank_bits(bits);
            high_rank = utils::high_rank_from_rank_bits(bits);
            self.flush_lookup.insert(prime_product, Meta {
                hand_rank: rank_suited,
                card_rank: high_rank
            });
            self.unsuited_lookup.insert(prime_product, Meta {
                hand_rank: rank_unsuited,
                card_rank: high_rank,
            });
            rank_suited = rank_suited.wrapping_add(1);
            rank_unsuited = rank_unsuited.wrapping_add(1);
        }
    }

    fn multiples(&mut self) {
        let mut product;

        // Three of a kind
        let mut hand_rank = WORST_STRAIGHT_FLUSH + 1;

        for trips in INT_RANKS.rev() {
            product = PRIMES[trips as usize].wrapping_pow(3);
            self.unsuited_lookup.insert(product, Meta {
                hand_rank,
                card_rank: Rank::ALL_VARIANTS[trips as usize],
            });
            hand_rank = hand_rank.wrapping_add(1);
        }

        // Pair
        hand_rank = WORST_FLUSH + 1;

        for pair in INT_RANKS.rev() {
            let kickers = INT_RANKS.rev().filter(|&i| i != pair);
            for kicker in kickers {
                product = PRIMES[pair as usize].wrapping_pow(2).wrapping_mul(PRIMES[kicker as usize]);
                self.unsuited_lookup.insert(product, Meta {
                    hand_rank,
                    card_rank: Rank::ALL_VARIANTS[pair as usize],
                });
                hand_rank = hand_rank.wrapping_add(1);
            }
        }
    }
}

impl Default for LookupTable {
    fn default() -> Self {
        Self::new()
    }
}

pub mod constants {
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
}
