//! A module for generating decks of cards.

use itertools::Itertools;

use crate::{Card, Rank, Suit};

/// Generate an iterator that will yield every card in a standard 52-card
/// deck once. The order in which the cards are yielded is **not**
/// random.
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use poker::deck;
///
/// let deck: Vec<_> = deck::generate().collect();
/// assert_eq!(deck.len(), 52);
/// let mut unique_cards = HashSet::new();
/// for card in deck {
///     // `insert()` returns false if the item is already present
///     assert!(unique_cards.insert(card));
/// }
/// ```
pub fn generate() -> impl Iterator<Item = Card> {
    Rank::ALL_VARIANTS
        .iter()
        .cartesian_product(Suit::ALL_VARIANTS.iter())
        .map(|(&rank, &suit)| Card::new(rank, suit))
}

/// Like [`generate`], but generate a shuffled deck using
/// [`rand`] and returned a [`Vec`] of [`Card`]s.
#[cfg(feature = "rand")]
pub fn shuffled() -> Vec<Card> { shuffled_with(&mut rand::thread_rng()) }

/// Like [`shuffled`], but generate a shuffled deck
/// using anything that implements [`rand::Rng`].
#[cfg(feature = "rand")]
pub fn shuffled_with<R>(rng: &mut R) -> Vec<Card>
where
    R: rand::Rng + ?Sized,
{
    use rand::prelude::*;
    let mut deck = generate().collect::<Vec<_>>();
    deck.shuffle(rng);
    deck
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn card_integers_unique() {
        let deck = generate();
        let mut ints = HashSet::with_capacity(52);
        deck.into_iter().for_each(|card| {
            assert!(ints.insert(card.unique_integer()));
        });
        assert_eq!(ints.len(), 52);
    }

    #[test]
    fn card_suit_and_rank_calculations() {
        let deck = generate();
        let mut suits = HashMap::with_capacity(4);
        let mut ranks = HashMap::with_capacity(13);
        for card in deck {
            let suit_count = suits.entry(card.suit()).or_insert(0);
            let rank_count = ranks.entry(card.rank()).or_insert(0);
            *suit_count += 1;
            *rank_count += 1;
        }
        assert!(suits.into_iter().all(|(_, count)| count == 13));
        assert!(ranks.into_iter().all(|(_, count)| count == 4));
    }

    #[test]
    fn generate_deck_is_52_cards() {
        assert_eq!(generate().count(), 52);
    }

    #[test]
    #[cfg(feature = "rand")]
    fn generate_shuffled_deck_is_52_cards() {
        assert_eq!(shuffled().len(), 52);
    }
}
