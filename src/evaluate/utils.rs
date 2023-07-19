use std::array;

use crate::{
    card::{rank::Rank, Card},
    constants::{INT_RANKS, PRIMES},
    evaluate::lookup_table, Suit,
};

#[derive(Debug, Clone, Copy)]
struct Combinations<'a, T, const N: usize> {
    data: &'a [T],
    indices: [usize; N],
    done: bool,
}

impl<'a, T, const N: usize> Combinations<'a, T, N> {
    fn new(data: &'a [T]) -> Self {
        let indices = array::from_fn(|index| index);
        Self {
            data,
            indices,
            done: false,
        }
    }
}

impl<'a, T: Copy, const N: usize> Iterator for Combinations<'a, T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.indices.map(|i| self.data[i]);

        for i in (0..N).rev() {
            if i == 0 && self.indices[i] == self.data.len() - N + i {
                self.done = true;
            }

            if self.indices[i] < self.data.len() - N + i {
                self.indices[i] += 1;
                for j in i + 1..N {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                break;
            }
        }

        Some(result)
    }
}

/// Originally from <http://www-graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation>.
/// This differs from the implementation in Python because we use trailing
/// zeroes.
#[derive(Debug, Clone, Copy)]
pub struct BitSequence {
    bits: i16,
    t: i16,
    next_bits: i16,
}

impl BitSequence {
    pub const fn new(bits: i16) -> Self {
        Self {
            bits,
            t: 0,
            next_bits: 0,
        }
    }

    pub fn get_next(&mut self) -> i16 {
        self.t = self.bits | self.bits.wrapping_sub(1);
        self.next_bits = self.t.wrapping_add(1);
        self.next_bits |= ((!self.t & (!self.t).wrapping_neg()).wrapping_sub(1))
            >> self.bits.trailing_zeros().wrapping_add(1);
        self.bits = self.next_bits;
        self.next_bits
    }
}

pub fn const_combos<T, const N: usize>(items: &[T]) -> impl Iterator<Item = [T; N]> + '_
where
    T: Copy,
{
    Combinations::new(items)
}

/// Calculate a hand's prime product by using it's bit rank representation.
pub fn prime_product_from_rank_bits(rank_bits: i16) -> i32 {
    let mut product: i32 = 1;
    for i in INT_RANKS {
        // Check to see if the bit for a given rank is turned on
        if rank_bits & (1 << i) != 0 {
            // If so, we multiply in the prime number corresponding to that rank
            product = product.wrapping_mul(PRIMES[i as usize]);
        }
    }
    product
}

/// Calculate a hand's prime product if an entire `Card` representation is
/// available.
pub fn prime_product_from_hand(hand: [Card; 5]) -> i32 {
    hand.into_iter()
        .map(|card| card.unique_integer() & 0xFF)
        .fold(1, |acc, x| acc.wrapping_mul(x))
}

/// Obtain the high card from a given set of rank bits bit-ORed together.
pub fn high_rank_from_rank_bits(rank_bits: i16) -> Rank {
    // We don't want to return an Ace as the high card if it's a five-high straight
    if rank_bits == lookup_table::constants::STRAIGHTS[9] {
        return Rank::Five;
    }
    for i in INT_RANKS.rev() {
        if rank_bits & (1 << i) != 0 {
            return Rank::ALL_VARIANTS[i as usize];
        }
    }
    unreachable!();
}

/// Verify that all cards in a slice are unique.
pub fn all_unique(hand: &[Card]) -> bool {
    let mut card_flags = 0u64;
    for &card in hand {
        let card_flag = 1u64 << card_to_index(card);
        if card_flags & card_flag != 0 {
            return false;
        }
        card_flags |= card_flag;
    }
    true
}

// Given a card, will return a unique index from 0 to 51, inclusive.
fn card_to_index(card: Card) -> u8 {
    let suit_shift = match card.suit() {
        Suit::Clubs => 0,
        Suit::Diamonds => 13,
        Suit::Hearts => 26,
        Suit::Spades => 39,
    };
    
    let rank_shift = match card.rank() {
        Rank::Two => 0,
        Rank::Three => 1,
        Rank::Four => 2,
        Rank::Five => 3,
        Rank::Six => 4,
        Rank::Seven => 5,
        Rank::Eight => 6,
        Rank::Nine => 7,
        Rank::Ten => 8,
        Rank::Jack => 9,
        Rank::Queen => 10,
        Rank::King => 11,
        Rank::Ace => 12,
    };
    
    suit_shift + rank_shift
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards;

    #[test]
    fn bit_sequence_generator_works() {
        let some_number = 0b10011;
        let mut xs = BitSequence::new(some_number);

        let mut next_check = move |bin: i16| {
            assert_eq!(xs.get_next(), bin);
        };

        next_check(0b00010101);
        next_check(0b00010110);
        next_check(0b00011001);
        next_check(0b00011010);
        next_check(0b00011100);
        next_check(0b00100011);
    }

    #[test]
    fn check_all_unique() {
        let cards: Vec<_> = cards!["Th", "Td", "Th"].try_collect().unwrap();
        assert!(!all_unique(&cards));
        let cards: Vec<_> = cards!["Th", "Th", "Td"].try_collect().unwrap();
        assert!(!all_unique(&cards));
        let cards: Vec<_> = cards!["Th", "5c", "3d", "Th"].try_collect().unwrap();
        assert!(!all_unique(&cards));
        let cards: Vec<_> = cards!["5c", "Th", "3d", "Th"].try_collect().unwrap();
        assert!(!all_unique(&cards));
    }

    #[test]
    fn const_combos_works() {
        let combos = Combinations::<'_, _, 2>::new(&vec!['c', 'a', 't']).collect::<Vec<_>>();
        dbg!(&combos);
        let expected_combos: [[char; 2]; 3] = [['c', 'a'], ['c', 't'], ['a', 't']];
        assert_eq!(combos.len(), expected_combos.len());
        for &combo in &expected_combos {
            assert!(combos.contains(&combo.into()));
        }
    }
}
