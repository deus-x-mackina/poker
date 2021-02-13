use std::{collections::HashMap, hash::BuildHasherDefault};

#[cfg(not(target_pointer_width = "64"))]
use hashers::fnv::FNV1aHasher32 as FNV;
#[cfg(target_pointer_width = "64")]
use hashers::fnv::FNV1aHasher64 as FNV;
use variter::VarIter;

use self::constants::*;
use crate::{
    card::rank::Rank,
    constants::{INT_RANKS, PRIMES},
    evaluate::{hand_rank::PokerHandRank, meta::Meta, utils},
};

type DefaultHasher = BuildHasherDefault<FNV>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LookupTable {
    pub(crate) flush_lookup: HashMap<i32, Meta, DefaultHasher>,
    pub(crate) unsuited_lookup: HashMap<i32, Meta, DefaultHasher>,
}

impl LookupTable {
    #[inline]
    pub fn new() -> Self {
        let mut table = Self {
            flush_lookup: HashMap::with_capacity_and_hasher(6175, DefaultHasher::default()),
            unsuited_lookup: HashMap::with_capacity_and_hasher(1287, DefaultHasher::default()),
        };
        table.flushes_straights_high_cards();
        table.multiples();
        table
    }

    fn flushes_straights_high_cards(&mut self) {
        let mut flushes = Vec::with_capacity(1277);
        let mut gen = utils::bit_sequence_generator(0b11111);

        for _ in 0..1286 {
            let f = gen.next().unwrap();
            let mut not_sf = true;
            for &sf in &STRAIGHT_FLUSHES {
                if f ^ sf == 0 {
                    not_sf = false;
                    break;
                }
            }
            if not_sf {
                flushes.push(f);
            }
        }

        let mut rank_suited = 1;
        let mut rank_unsuited = MAX_FLUSH + 1;
        let mut high_rank;
        let mut prime_product;

        // Straight flushes and straights
        for &sf in &STRAIGHT_FLUSHES {
            prime_product = utils::prime_product_from_rank_bits(sf);
            high_rank = utils::high_rank_from_rank_bits(sf);
            self.flush_lookup.insert(
                prime_product,
                Meta::StraightFlush {
                    hand_rank: PokerHandRank(rank_suited),
                    high_rank,
                },
            );
            self.unsuited_lookup.insert(
                prime_product,
                Meta::Straight {
                    hand_rank: PokerHandRank(rank_unsuited),
                    high_rank,
                },
            );
            rank_suited += 1;
            rank_unsuited += 1;
        }

        // Flushes and high cards
        rank_suited = MAX_FULL_HOUSE + 1;
        rank_unsuited = MAX_PAIR + 1;
        for f in flushes.into_iter().rev() {
            prime_product = utils::prime_product_from_rank_bits(f);
            high_rank = utils::high_rank_from_rank_bits(f);
            self.flush_lookup.insert(
                prime_product,
                Meta::Flush {
                    hand_rank: PokerHandRank(rank_suited),
                    high_rank,
                },
            );
            self.unsuited_lookup.insert(
                prime_product,
                Meta::HighCard {
                    hand_rank: PokerHandRank(rank_unsuited),
                    high_rank,
                },
            );
            rank_suited += 1;
            rank_unsuited += 1;
        }
    }

    fn multiples(&mut self) {
        let backwards_ranks = INT_RANKS.rev();
        let mut product;

        // Four of a kind
        let mut rank = MAX_STRAIGHT_FLUSH + 1;
        for i in backwards_ranks.clone() {
            let kickers = backwards_ranks.clone().filter(|&kicker| kicker != i);
            for k in kickers {
                product = PRIMES[i as usize].wrapping_pow(4) * PRIMES[k as usize];
                self.unsuited_lookup.insert(
                    product,
                    Meta::FourOfAKind {
                        hand_rank: PokerHandRank(rank),
                        quads: Rank::ALL_VARIANTS[i as usize],
                    },
                );
                rank += 1;
            }
        }

        // Full house
        rank = MAX_FOUR_OF_A_KIND + 1;
        for i in backwards_ranks.clone() {
            let pair_ranks = backwards_ranks.clone().filter(|&pr| pr != i);
            for pr in pair_ranks {
                product = PRIMES[i as usize].wrapping_pow(3) * PRIMES[pr as usize].wrapping_pow(2);
                self.unsuited_lookup.insert(
                    product,
                    Meta::FullHouse {
                        hand_rank: PokerHandRank(rank),
                        pair: Rank::ALL_VARIANTS[pr as usize],
                        trips: Rank::ALL_VARIANTS[i as usize],
                    },
                );
                rank += 1;
            }
        }

        // Three of a kind
        rank = MAX_STRAIGHT + 1;
        for i in backwards_ranks.clone() {
            let kickers = backwards_ranks.clone().filter(|&kicker| kicker != i);
            let gen = utils::combinations_generator(kickers, 2);
            for k in gen {
                let c1 = k[0] as usize;
                let c2 = k[1] as usize;
                product = PRIMES[i as usize].wrapping_pow(3) * PRIMES[c1] * PRIMES[c2];
                self.unsuited_lookup.insert(
                    product,
                    Meta::ThreeOfAKind {
                        hand_rank: PokerHandRank(rank),
                        trips: Rank::ALL_VARIANTS[i as usize],
                    },
                );
                rank += 1;
            }
        }

        // Two pair
        rank = MAX_THREE_OF_A_KIND + 1;
        let two_pairs_combos = utils::combinations_generator(backwards_ranks.clone(), 2);
        for two_pair in two_pairs_combos {
            let pair1 = two_pair[0];
            let pair2 = two_pair[1];
            let kickers = backwards_ranks
                .clone()
                .filter(|&kicker| kicker != pair1 && kicker != pair2);
            for kicker in kickers {
                product = PRIMES[pair1 as usize].wrapping_pow(2)
                    * PRIMES[pair2 as usize].wrapping_pow(2)
                    * PRIMES[kicker as usize];
                self.unsuited_lookup.insert(
                    product,
                    Meta::TwoPair {
                        hand_rank: PokerHandRank(rank),
                        high_pair: Rank::ALL_VARIANTS[pair1 as usize],
                        low_pair: Rank::ALL_VARIANTS[pair2 as usize],
                    },
                );
                rank += 1;
            }
        }

        // Pair
        rank = MAX_TWO_PAIR + 1;
        for pair_rank in backwards_ranks.clone() {
            let kickers = backwards_ranks
                .clone()
                .filter(|&kicker| kicker != pair_rank);
            let kicker_combos = utils::combinations_generator(kickers, 3);
            for kicker_combo in kicker_combos {
                let k1 = kicker_combo[0] as usize;
                let k2 = kicker_combo[1] as usize;
                let k3 = kicker_combo[2] as usize;
                product = PRIMES[pair_rank as usize].wrapping_pow(2)
                    * PRIMES[k1]
                    * PRIMES[k2]
                    * PRIMES[k3];
                self.unsuited_lookup.insert(
                    product,
                    Meta::Pair {
                        hand_rank: PokerHandRank(rank),
                        pair: Rank::ALL_VARIANTS[pair_rank as usize],
                    },
                );
                rank += 1;
            }
        }
    }
}

impl AsRef<LookupTable> for LookupTable {
    #[inline]
    fn as_ref(&self) -> &LookupTable { self }
}

pub(crate) mod constants {
    pub const MAX_STRAIGHT_FLUSH: i16 = 10;
    pub const MAX_FOUR_OF_A_KIND: i16 = 166;
    pub const MAX_FULL_HOUSE: i16 = 322;
    pub const MAX_FLUSH: i16 = 1599;
    pub const MAX_STRAIGHT: i16 = 1609;
    pub const MAX_THREE_OF_A_KIND: i16 = 2467;
    pub const MAX_TWO_PAIR: i16 = 3325;
    pub const MAX_PAIR: i16 = 6185;
    pub const MAX_HIGH_CARD: i16 = 7462;

    pub const STRAIGHT_FLUSHES: [i16; 10] = [
        0b1111100000000, // 7936
        0b111110000000,  // 3968
        0b11111000000,   // 1984
        0b1111100000,    // 992
        0b111110000,     // 496
        0b11111000,      // 248
        0b1111100,       // 124
        0b111110,        // 62
        0b11111,         // 31
        0b1000000001111, // 4111
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_metadata_ordering() {
        let LookupTable {
            unsuited_lookup, ..
        } = LookupTable::new();
        for (_, metadata) in unsuited_lookup {
            if let Meta::TwoPair {
                high_pair,
                low_pair,
                ..
            } = metadata
            {
                assert!(high_pair > low_pair);
            }
        }
    }
}
