//! Create and manage poker cards with suits and ranks.
//!
//! The way `poker` encodes cards is the key to how poker hands are evaluated.
//! At their core, each card is represented as a single 32-bit integer that
//! encodes all the information we need about it. Using an integer type has many
//! advantages, especially in the context of Rust — the `Card` type
//! is lightweight, it is `Copy`, and all operations needed to extract
//! information from the integer are simple enough to execute within a `const
//! fn`.
//!
//! ```text
//! Card:
//!                           bitrank     suit rank   prime
//!                     +--------+--------+--------+--------+
//!                     |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
//!                     +--------+--------+--------+--------+
//! ```
//! (Adapted from [`treys`], originally from [Cactus Kev])
//!
//! - `b`: The first 16 bits, of which 13 are used, represent bit flags that
//!   indicate the card's rank. The rightmost bit being turned on represent a
//!   deuce, and the leftmost a king. Only one bit should be turned on at a time
//!   for any one card.
//! - `cdhs`: Four bitflags that represent the suit of a given card, where `c` =
//!   clubs, `d` = diamonds, `h` = hearts, and `s` = spades.
//! - `r`: Four bits of spaces meant for storing a number from 0 to 12,
//!   representing the rank of the card, where 0 = deuce, 1 = trey, ..., 12 =
//!   ace.
//! - `p`: Six bits of space meant to hold a prime number that corresponds to
//!   the cards rank. A deuce is coded with the first prime number (2), a trey
//!   with 3, up until an ace with 41.
//!
//! This representation may seem redundant, but the different representations
//! are useful during different parts of the evaluation process. Given this, we
//! can be sure of the following example:
//!
//! ```
//! use poker::card;
//!
//! let ace_of_spades = card!(Ace, Spades);
//! assert_eq!(
//!     ace_of_spades.unique_integer(),
//!     // -A-------------|---S|-12-|---42---
//!     0b0010000_00000000_0001_1100_00101001
//! );
//! ```
//! You will rarely need to work with the [`unique_integer()`] directly, but it
//! may be helpful for debugging.
//!
//! [`treys`]: https://github.com/ihendley/treys/blob/master/treys/evaluator.py
//! [Cactus Kev]: http://suffe.cool/poker/evaluator.html
//! [`unique_integer()`]: Card::unique_integer

mod macros;
pub(crate) mod rank;
pub(crate) mod suit;

use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    fmt,
    hash::Hash,
    iter::{FromIterator, FusedIterator},
    str::FromStr,
};

use itertools::Itertools;
use variter::VarIter;

#[doc(inline)]
pub use self::{rank::Rank, suit::Suit};
use crate::{constants::PRIMES, error::ParseCardError};

/// A single playing card.
///
/// Some things to note:
/// - There are multiple ways to create singular and multiple cards besides
///   [`Card::new`]
/// - When printed in [`Display`] mode, cards are printed to look like physical
///   cards.
/// - Joker cards are not supported.
///
/// # Example
///
/// You can create a card using the verbose [`Card::new`] constructor. This
/// constructor qualifies as a `const fn`:
///
/// ```
/// use poker::{Card, Rank, Suit};
///
/// const ACE_OF_SPADES: Card = Card::new(Rank::Ace, Suit::Spades);
///
/// // `to_string()` converts a value to its `Display` form
/// assert_eq!(ACE_OF_SPADES.to_string(), "[ A♠ ]");
/// ```
///
/// [`Display`]: std::fmt::Display
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    unique_integer: i32,
}

impl Card {
    /// Create a new, singular [`Card`] given a [`Rank`] and a [`Suit`] variant.
    /// This constructor is verbose, but explicit. It is not often that you
    /// need to construct a single [`Card`], but other functions for
    /// conveniently creating [`Card`]s rely on this one.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Card, Rank, Suit};
    ///
    /// let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
    /// let card_display = three_of_clubs.to_string();
    /// assert_eq!(card_display, "[ 3♣ ]");
    /// println!("Is this your card? {}", card_display);
    /// ```
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        let rank_int = rank.as_i32();
        let suit_int = suit.as_i32();
        let rank_prime = PRIMES[rank_int as usize];
        let bit_rank = (1 << rank_int) << 16;
        let card_suit = suit_int << 12;
        let card_rank = rank_int << 8;
        Self {
            unique_integer: bit_rank | card_suit | card_rank | rank_prime,
        }
    }

    /// Try to create a single [`Card`] using [`char`] types instead of [`Rank`]
    /// and [`Suit`] enumeration types.
    ///
    /// # Errors
    ///
    /// This function **will** fail with a [`ParseCardError`] if `rank_char` is
    /// anything other than one of '`23456789TJQKA`', and `suit_char` is
    /// anything other than one of '`chsd`'. This is case-sensitive!
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Card, Rank, Suit};
    ///
    /// let card_one = Card::new(Rank::Eight, Suit::Diamonds);
    /// let card_two = Card::try_from_chars('8', 'd').expect("invalid rank or suit character");
    /// assert_eq!(card_one, card_two);
    /// ```
    pub fn try_from_chars(rank_char: char, suit_char: char) -> Result<Self, ParseCardError> {
        let rank = rank_char
            .try_into()
            .map_err(|incorrect_char| ParseCardError::InvalidRank {
                original_input: rank_char.to_string(),
                incorrect_char,
            })?;
        let suit = suit_char
            .try_into()
            .map_err(|incorrect_char| ParseCardError::InvalidSuit {
                original_input: suit_char.to_string(),
                incorrect_char,
            })?;
        Ok(Self::new(rank, suit))
    }

    /// Obtain this [`Card`]'s rank, which is one of clubs, hearts, diamonds, or
    /// spades.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Card, Rank, Suit};
    ///
    /// let some_card = Card::new(Rank::Queen, Suit::Hearts);
    /// assert_eq!(some_card.rank(), Rank::Queen);
    /// ```
    pub const fn rank(self) -> Rank {
        let rank_int = (self.unique_integer >> 8) & 0xF;
        Rank::from_i32(rank_int)
    }

    /// Obtain this [`Card`]'s suit, which is one of two, three, four, five,
    /// six, seven, eight, nine, ten, jack, queen, king, or ace.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Card, Rank, Suit};
    ///
    /// let some_card = Card::new(Rank::King, Suit::Diamonds);
    /// assert_eq!(some_card.suit(), Suit::Diamonds);
    /// ```
    pub const fn suit(self) -> Suit {
        let suit_int = (self.unique_integer >> 12) & 0xF;
        Suit::from_i32(suit_int)
    }

    /// Obtain this [`Card`]'s unique integer encoding, which distinguishes it
    /// from other cards. See the [module level documentation] for more
    /// about what this number encodes.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Card, Rank, Suit};
    ///
    /// const ACE_OF_SPADES: Card = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(
    ///     ACE_OF_SPADES.unique_integer(),
    ///     0b0010000_00000000_00011100_00101001
    /// );
    /// ```
    ///
    /// [module level documentation]: self
    pub const fn unique_integer(self) -> i32 { self.unique_integer }

    /// Obtain a two-character [`String`] representation of this [`Card`]. This
    /// will be in the same format that other `Card`-producing parsing
    /// functions accept.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Card, Rank, Suit};
    ///
    /// let card_one = Card::new(Rank::Seven, Suit::Clubs);
    /// let card_one_string = card_one.rank_suit_string();
    /// assert_eq!(card_one_string, "7c");
    /// let card_two = card_one_string.parse().expect("couldn't parse string");
    /// assert_eq!(card_one, card_two);
    /// ```
    pub fn rank_suit_string(self) -> String {
        let mut s = String::with_capacity(2);
        s.push(self.rank().as_char());
        s.push(self.suit().as_char());
        s
    }

    /// Generate an iterator that will yield every card in a standard 52-card
    /// deck once. The order in which the cards are yielded is **not**
    /// random.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// use poker::Card;
    ///
    /// let deck: Vec<_> = Card::generate_deck().collect();
    /// assert_eq!(deck.len(), 52);
    /// let mut unique_cards = HashSet::new();
    /// for card in deck {
    ///     // `insert()` returns false if the item is already present
    ///     assert!(unique_cards.insert(card));
    /// }
    /// ```
    pub fn generate_deck() -> impl Iterator<Item = Self> {
        Rank::ALL_VARIANTS
            .iter()
            .cartesian_product(Suit::ALL_VARIANTS.iter())
            .map(|(&rank, &suit)| Self::new(rank, suit))
    }

    /// Like [`Card::generate_deck`], but generate a shuffled deck using
    /// [`rand`] and returned a boxed slice of [`Card`]s.
    #[cfg(feature = "rand")]
    pub fn generate_shuffled_deck() -> Vec<Self> {
        Self::generate_shuffled_deck_with(&mut rand::thread_rng())
    }

    /// Like [`Card::generate_shuffled_deck`], but generate a shuffled deck
    /// using anything that implements [`rand::Rng`].
    #[cfg(feature = "rand")]
    pub fn generate_shuffled_deck_with<R>(mut rng: &mut R) -> Vec<Card>
    where
        R: rand::Rng + ?Sized,
    {
        use rand::prelude::*;
        let mut deck = Self::generate_deck().collect::<Vec<_>>();
        deck.shuffle(&mut rng);
        deck
    }

    /// From an [`Iterator`] that yields strings, return a new [`Iterator`] that
    /// yields `Result<Card, ParseCardError>`. The iterator adaoptor returned by
    /// this associated function has a special method [`try_collect`], which
    /// is a shortcut over using `collect::<Result<_, _>, _>()`. This was
    /// inspired by the [`itertools`] crate.
    ///
    /// # Errors
    ///
    /// The returned iterator will yield a [`ParseCardError`] if one of the
    /// strings encountered is not:
    /// - exactly two characters in length
    /// - contains one of '`23456789TJQKA`' followed by one of '`chsd`'. This is
    ///   case-sensitive!
    ///
    /// This implementation is not short-circuiting and you will be responsible
    /// for dealing with the `Result`s.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::{Card, Rank, Suit};
    /// let cards = Card::parse_to_iter("As Ad".split_whitespace())
    ///     .try_collect::<Vec<_>>()
    ///     .expect("couldn't parse cards");
    /// assert_eq!(
    ///     cards,
    ///     [
    ///         Card::new(Rank::Ace, Suit::Spades),
    ///         Card::new(Rank::Ace, Suit::Diamonds)
    ///     ]
    /// );
    /// ```
    ///
    /// [`try_collect`]: ParseToIter::try_collect
    /// [`itertools`]: itertools::Itertools::try_collect
    pub fn parse_to_iter<S>(
        strings: S,
    ) -> ParseToIter<impl Iterator<Item = Result<Self, ParseCardError>>>
    where
        S: IntoIterator,
        S::Item: AsRef<str>,
    {
        ParseToIter(strings.into_iter().map(|s| s.as_ref().parse()))
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut chars = value.chars();
        // Only parse input of two chars -- no more, no less!
        if let (Some(rank), Some(suit), None) = (chars.next(), chars.next(), chars.next()) {
            let rank =
                Rank::try_from(rank).map_err(|incorrect_char| ParseCardError::InvalidRank {
                    original_input: value.to_string(),
                    incorrect_char,
                })?;
            let suit =
                Suit::try_from(suit).map_err(|incorrect_char| ParseCardError::InvalidSuit {
                    original_input: value.to_string(),
                    incorrect_char,
                })?;
            Ok(Self::new(rank, suit))
        } else {
            Err(ParseCardError::InvalidLength {
                original_input: value.to_string(),
            })
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering { self.rank().cmp(&other.rank()) }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card")
            .field("unique_integer", &self.unique_integer())
            .field("rank", &self.rank())
            .field("suit", &self.suit())
            .finish()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {}{} ]", self.rank(), self.suit())
    }
}

/// An iterator adaptor returned from [`Card::parse_to_iter`]. It doesn't do
/// anything special, but does have a method
/// [`try_collect`](ParseToIter::try_collect) to consolidate [`Card`]s into a
/// collection, or fail upon the first error encountered.
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ParseToIter<I>(I);

impl<I: Iterator> Iterator for ParseToIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> { self.0.next() }

    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.0.fold(init, f)
    }
}

impl<I: FusedIterator> FusedIterator for ParseToIter<I> {}

impl<I: ExactSizeIterator> ExactSizeIterator for ParseToIter<I> {
    fn len(&self) -> usize { self.0.len() }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for ParseToIter<I> {
    fn next_back(&mut self) -> Option<Self::Item> { self.0.next_back() }
}

impl<I, T, E> ParseToIter<I>
where
    I: Iterator<Item = Result<T, E>>,
{
    /// A shortcut over calling `collect::<Result<_, _>, _>()`. Inspired by the
    /// [`itertools`] crate.
    ///
    /// # Errors
    ///
    /// If any item in this iterator yields an `Err` variant, that `Err` is
    /// returned.
    ///
    /// [`itertools`]: itertools::Itertools::try_collect
    pub fn try_collect<C: FromIterator<T>>(self) -> Result<C, E> { self.0.collect() }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn from_str_fails() {
        let mut result: Result<Card, ParseCardError>;
        result = "2x".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseCardError::InvalidSuit {
                original_input: "2x".into(),
                incorrect_char: 'x'
            }
        );

        result = "Rd".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseCardError::InvalidRank {
                original_input: "Rd".into(),
                incorrect_char: 'R'
            }
        );

        result = "AsX".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseCardError::InvalidLength {
                original_input: "AsX".into()
            }
        );
    }

    #[test]
    fn try_from_works() {
        for (rank_index, rank) in Rank::ALL_VARIANTS.iter().map(|r| r.as_char()).enumerate() {
            for (suit_index, suit) in Suit::ALL_VARIANTS.iter().map(|s| s.as_char()).enumerate() {
                let card_string = rank.to_string() + suit.to_string().as_str();
                let result = card_string.parse();
                assert_eq!(
                    result,
                    Ok(Card::new(
                        Rank::ALL_VARIANTS[rank_index],
                        Suit::ALL_VARIANTS[suit_index]
                    ))
                );
            }
        }
    }

    #[test]
    fn card_integers_unique() {
        let deck = Card::generate_deck();
        let mut ints = HashSet::with_capacity(52);
        deck.into_iter().for_each(|card| {
            assert!(ints.insert(card.unique_integer));
        });
        assert_eq!(ints.len(), 52);
    }

    #[test]
    fn card_suit_and_rank_calculations() {
        let deck = Card::generate_deck();
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
        assert_eq!(Card::generate_deck().count(), 52);
    }

    #[test]
    #[cfg(feature = "rand")]
    fn generate_shuffled_deck_is_52_cards() {
        assert_eq!(Card::generate_shuffled_deck().len(), 52);
    }
}
