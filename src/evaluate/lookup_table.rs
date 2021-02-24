use std::{collections::HashMap, hash::BuildHasherDefault};

// Use FNV1aHasher64 on 64-bit systems. It hashed much faster.
// SAFETY: Non-cryptographic algorithms can be used here because it improves hashing
// performance and the hash tables are not available to mutate in the public interface.
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

/// Stores information about looking up poker hands.
///
/// There are two hash tables, one for hands where the cards are suited (flushes
/// and straight flushes) and one for hands where the cards are not suited (the
/// rest of the poker hands). Both tables are indexed by a hand's prime product.
///
/// For example, the worst possible hand is 23457 (unsuited). The prime product
/// of these ranks is 2 * 3 * 5 * 7 * 13 = 2730. The evaluation implementation
/// first checks to make sure the hand is not suited, then indexes into the
/// unsuited lookup to find that `unsuited_lookup\[2730\]` is equal
/// to `Meta::HighCard { hand_rank: HandRank(7462), high_rank: Rank::Seven }`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LookupTable {
    pub flush_lookup: HashMap<i32, Meta, DefaultHasher>,
    pub unsuited_lookup: HashMap<i32, Meta, DefaultHasher>,
}

impl LookupTable {
    pub fn new() -> Self {
        let mut table = Self {
            flush_lookup: HashMap::with_capacity_and_hasher(6175, DefaultHasher::default()),
            unsuited_lookup: HashMap::with_capacity_and_hasher(1287, DefaultHasher::default()),
        };
        table.flushes_straights_high_cards();
        table.multiples();
        table
    }

    /// Calculate the metadata for flushes, straights, high cards, and straight
    /// flushes.
    fn flushes_straights_high_cards(&mut self) {
        let mut not_straights = Vec::with_capacity(1277);
        // `0b11111` represents a 2-3-4-5-6 hand (suit unknown).
        // the bit sequence generator will help generate other permutations
        // of integers where only five bits are turned on, thus allowing us to consider
        // every hand combination of five ranks together where each rank is unique,
        // i.e., there are no pairs, trips, quads, etc.
        // We are going to add each combination that isn't a straight into the
        // `not_straights` vector
        let mut gen = utils::bit_sequence_generator(0b11111);

        // info: We statically calculated all straights in the `STRAIGHTS` constant
        for _ in 0..1286 {
            let bits = gen.next().unwrap();
            let mut not_straight = true;
            for &straight in &STRAIGHTS {
                // If the bits XOR a straight is 0, then it **is** a s traight, so we don't add
                // it to our vector
                if bits ^ straight == 0 {
                    not_straight = false;
                    break;
                }
            }
            if not_straight {
                not_straights.push(bits);
            }
        }

        // Now we have `STRAIGHTS` (our constant) and `not_straights` (dynamically
        // calculated). Using these, we can consider both sets as if the ranks
        // they encode are suited
        //   - `STRAIGHT` hands suited become straight flushes
        //   - `not_straight` hands become flushes
        // We can also consider them unsuited:
        //   - `STRAIGHT` hands are just straights
        //   - `not_straight` hands are high-card hands (pairs, etc. not possible)

        // Let's first work with the `STRAIGHTS`.

        // If suited, we start with the best hand, a royal flush. This corresponds to a
        // value of 1
        let mut rank_suited = 1;

        // If unsuited, we start with the best possible straight, which is one worse
        // (1+) the worst (max) flush
        let mut rank_unsuited = WORST_FLUSH + 1;

        // These are recycled and hold the rank of the highest card of the hand and the
        // prime product
        let mut high_rank;
        let mut prime_product;

        // Straight flushes and straights
        for &straight in &STRAIGHTS {
            // We get the prime product using the bits
            prime_product = utils::prime_product_from_rank_bits(straight);

            // We also obtain the highest rank from the bits
            high_rank = utils::high_rank_from_rank_bits(straight);

            // Into the flush table we map the prime product to a straight flush
            // the has our current `rank_suited` value and the highest card's rank
            self.flush_lookup.insert(
                prime_product,
                Meta::StraightFlush {
                    hand_rank: PokerHandRank(rank_suited),
                    high_rank,
                },
            );

            // Into the unsuited table, we map the same prime product to a straight
            // with our current `rank_unsuited` value and the highest card's rank
            self.unsuited_lookup.insert(
                prime_product,
                Meta::Straight {
                    hand_rank: PokerHandRank(rank_unsuited),
                    high_rank,
                },
            );

            // We increment our values as in the next loop we consider the next-worse hand.
            rank_suited += 1;
            rank_unsuited += 1;
        }

        // Now, we work with our `not_straights`.

        // If suited, we have a flush, which is starts just worse than the worst full
        // house
        rank_suited = WORST_FULL_HOUSE + 1;

        // If unsuited, we start just worse than the worst pair
        rank_unsuited = WORST_PAIR + 1;

        // Flushes and high cards
        // We reverse `not_straights` before looping because we generated the worst
        // hands first, but we want to start mapping from the best hands
        for bits in not_straights.into_iter().rev() {
            // Get the prime product from the bits
            prime_product = utils::prime_product_from_rank_bits(bits);

            // Get the highest card's rank
            high_rank = utils::high_rank_from_rank_bits(bits);

            // In the flush table, map the prime product to a flush
            self.flush_lookup.insert(
                prime_product,
                Meta::Flush {
                    hand_rank: PokerHandRank(rank_suited),
                    high_rank,
                },
            );

            // In the unsuited table, map it to a high card hand
            self.unsuited_lookup.insert(
                prime_product,
                Meta::HighCard {
                    hand_rank: PokerHandRank(rank_unsuited),
                    high_rank,
                },
            );

            // Increment our values to consider the next worst hand
            rank_suited += 1;
            rank_unsuited += 1;
        }
    }

    /// Calculate metadata for all hands where at least one rank is repeated.
    fn multiples(&mut self) {
        // We want backwards ranks so we can consider the best/highest card ranks first
        let backwards_ranks = INT_RANKS.rev();

        // Reusable product holder
        let mut product;

        // Four of a kind
        // Given a four of a kind hand, we know one rank is repeated four times, and one
        // extra card, the kicker, is left, which can be one of the other eleven
        // card ranks. We start our rank at just worse than the worst straight
        // flush
        let mut rank = WORST_STRAIGHT_FLUSH + 1;

        // First, select our rank that there will be 4x
        for quad in backwards_ranks.clone() {
            // Then filter out our 4x rank so we can consider each possible kicker
            let kickers = backwards_ranks.clone().filter(|&kicker| kicker != quad);
            for k in kickers {
                // Get the prime product by hand, using `pow` if/when the card is present more
                // than once
                product = PRIMES[quad as usize].wrapping_pow(4) * PRIMES[k as usize];

                // Map the product to the appropriate hand
                self.unsuited_lookup.insert(
                    product,
                    Meta::FourOfAKind {
                        hand_rank: PokerHandRank(rank),
                        quads: Rank::ALL_VARIANTS[quad as usize],
                    },
                );
                rank += 1;
            }
        }

        // Full house
        // We have one three of a kind (trips) and one (pair)
        rank = WORST_FOUR_OF_A_KIND + 1;
        // We select our trips rank...
        for trips in backwards_ranks.clone() {
            // ...and select our pair rank
            let pair_ranks = backwards_ranks.clone().filter(|&pr| pr != trips);
            for pr in pair_ranks {
                // And we calculate the prime product using power of 3 for the 3x rank and power
                // of 2 for the 2x rank
                product =
                    PRIMES[trips as usize].wrapping_pow(3) * PRIMES[pr as usize].wrapping_pow(2);
                self.unsuited_lookup.insert(
                    product,
                    Meta::FullHouse {
                        hand_rank: PokerHandRank(rank),
                        pair: Rank::ALL_VARIANTS[pr as usize],
                        trips: Rank::ALL_VARIANTS[trips as usize],
                    },
                );
                rank += 1;
            }
        }

        // Three of a kind
        // One 3x rank and two unique kickers
        rank = WORST_STRAIGHT + 1;
        for trips in backwards_ranks.clone() {
            let kickers = backwards_ranks.clone().filter(|&kicker| kicker != trips);
            // We want every combination of two kickers
            let gen = utils::combinations_generator(kickers, 2);
            for k in gen {
                // Pull our kickers from the generator
                let c1 = k[0] as usize;
                let c2 = k[1] as usize;

                // Calculate our prime product with power of 3 for the trips and simply
                // multiply in the two kickers' primes
                product = PRIMES[trips as usize].wrapping_pow(3) * PRIMES[c1] * PRIMES[c2];
                self.unsuited_lookup.insert(
                    product,
                    Meta::ThreeOfAKind {
                        hand_rank: PokerHandRank(rank),
                        trips: Rank::ALL_VARIANTS[trips as usize],
                    },
                );
                rank += 1;
            }
        }

        // Two pair
        // Two unique 2x cards and one unique kicker
        rank = WORST_THREE_OF_A_KIND + 1;

        // We want want every combination of two card ranks together to consider as our
        // two pair ranks
        let two_pairs_combos = utils::combinations_generator(backwards_ranks.clone(), 2);
        for two_pair in two_pairs_combos {
            // Pull our two pairs
            let pair1 = two_pair[0];
            let pair2 = two_pair[1];

            // Our kickers are any rank that isn't equal to one of our two pair ranks
            let kickers = backwards_ranks
                .clone()
                .filter(|&kicker| kicker != pair1 && kicker != pair2);

            for kicker in kickers {
                // Product is power of two for our two pair ranks, multiplied by kicker
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
        // 1 pair rank and three unique kickers
        rank = WORST_TWO_PAIR + 1;
        for pair_rank in backwards_ranks.clone() {
            let kickers = backwards_ranks
                .clone()
                .filter(|&kicker| kicker != pair_rank);

            // We want every combination of three unique ranks that aren't equal to our pair
            // rank
            let kicker_combos = utils::combinations_generator(kickers, 3);
            for kicker_combo in kicker_combos {
                let k1 = kicker_combo[0] as usize;
                let k2 = kicker_combo[1] as usize;
                let k3 = kicker_combo[2] as usize;

                // Our product is the pair rank's prime to the power of two times the kickers'
                // primes
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

        // And we're done! Phew!
    }
}

pub mod constants {

    // These are the worst hand ranks for each of the poker hands

    pub const WORST_STRAIGHT_FLUSH: i16 = 10;
    pub const WORST_FOUR_OF_A_KIND: i16 = 166;
    pub const WORST_FULL_HOUSE: i16 = 322;
    pub const WORST_FLUSH: i16 = 1599;
    pub const WORST_STRAIGHT: i16 = 1609;
    pub const WORST_THREE_OF_A_KIND: i16 = 2467;
    pub const WORST_TWO_PAIR: i16 = 3325;
    pub const WORST_PAIR: i16 = 6185;
    pub const WORST_HIGH_CARD: i16 = 7462;

    /// Statically calculated bit straights
    pub const STRAIGHTS: [i16; 10] = [
        0b1111100000000, // 7936 => TJQKA
        0b0111110000000, // 3968 => 9TJQK
        0b0011111000000, // 1984 => 89TJQ
        0b0001111100000, // 992 => 789TJ
        0b0000111110000, // 496 => 6789T
        0b0000011111000, // 248 => 56789
        0b0000001111100, // 124 => 45678
        0b0000000111110, // 62 => 34567
        0b0000000011111, // 31 => 23456
        0b1000000001111, // 4111 => A2345
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
