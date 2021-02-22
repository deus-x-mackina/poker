//! Two different error types that may be encountered when trying to parse
//! [`Card`] types from strings, or when trying to evaluate hands.
//!
//! The [`Debug`] representations aren't *particularly* helpful, so try to
//! display errors as [`Display`](std::fmt::Display) when possible.

use std::{error::Error, fmt};

use itertools::Itertools;

use crate::card::Card;

/// An error than can be thrown when parsing [`Card`] types from strings.
///
/// # Examples
///
/// An input with invalid length yields [`ParseCardError::InvalidLength`].
///
/// ```
/// use poker::{Card, ParseCardError};
/// let ten_of_clubs = "10c"; // not two characters; use 'T' instead of '10'!
/// let result = ten_of_clubs.parse::<Card>();
/// assert_eq!(
///     result,
///     Err(ParseCardError::InvalidLength {
///         original_input: "10c".into()
///     })
/// );
/// ```
///
/// If an input *is* two characters, [`ParseCardError::InvalidRank`] will be
/// thrown if the first character, representing the card's rank, is not one of
/// '23456789TJQKA'.
///
/// ```
/// use poker::{Card, ParseCardError};
/// let jack_of_clubs = "jc";
/// let result = jack_of_clubs.parse::<Card>();
/// assert_eq!(
///     result,
///     Err(ParseCardError::InvalidRank {
///         original_input: "jc".into(),
///         incorrect_char: 'j'
///     })
/// );
/// ```
///
/// Similarly, a two-character input, with a second character representing the
/// suit, does not contain one of "chsd", [`ParseCardError::InvalidSuit`] will
/// be thrown.
///
/// ```
/// use poker::{Card, ParseCardError};
/// let two_of_diamonds = "2D";
/// let result = two_of_diamonds.parse::<Card>();
/// assert_eq!(
///     result,
///     Err(ParseCardError::InvalidSuit {
///         original_input: "2D".into(),
///         incorrect_char: 'D'
///     })
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseCardError {
    /// A string to be interpreted as a [`Card`] must be exactly two characters
    /// long. This variant is used if the input does not have meet this
    /// criterion.
    InvalidLength {
        /// The input that incited this error, converted to a [`String`] if
        /// needed. The original input is also used for calculating and
        /// reporting the incorrect number of characters received.
        original_input: String,
    },
    /// A string to be interpreted as a [`Card`] must have its first character
    /// be one of "23456789TJQKA", otherwise, this error will be thrown and
    /// indicate the incorrect character.
    InvalidRank {
        /// The input that incited this error, converted to a [`String`] if
        /// needed.
        original_input: String,
        /// The actual character within the input that was unexpected and could
        /// not be interpreted as a card rank.
        incorrect_char: char,
    },
    /// A string to be interpreted as a [`Card`] must have its first character
    /// be one of "23456789TJQKA", otherwise, this error will be thrown and
    /// indicate the incorrect character.
    InvalidSuit {
        /// The input that incited this error, converted to a [`String`] if
        /// needed.
        original_input: String,
        /// The actual character within the input that was unexpected and could
        /// not be interpreted as a card suit.
        incorrect_char: char,
    },
}

impl fmt::Display for ParseCardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength { original_input } => write!(
                f,
                "Error parsing input '{}' as a Card: Found input of length {}, expected 2",
                original_input,
                original_input.len()
            ),
            Self::InvalidRank {
                original_input,
                incorrect_char,
            } => write!(
                f,
                "Error parsing input '{}' as a Card: Invalid rank character '{}', expected one of \
                 [23456789TJQKA]",
                original_input, incorrect_char
            ),
            Self::InvalidSuit {
                original_input,
                incorrect_char,
            } => write!(
                f,
                "Error parsing input '{}' as a Card: Invalid suit character '{}', expected one of \
                 [chsd]",
                original_input, incorrect_char
            ),
        }
    }
}

impl Error for ParseCardError {}

/// An error that can be thrown when evaluating poker hands.
///
/// # Examples
///
/// If a group of [`Card`]s to be evaluated does not contain a set of unique
/// cards, [`EvalError::CardsNotUnique`] will be thrown, as shown below:
///
/// ```
/// use poker::{cards, Evaluator, EvalError};
/// let eval = Evaluator::new();
/// // Two king of clubs instead of a royal flush!
/// let hand = cards!(
///     Ten, Clubs;
///     Jack, Clubs;
///     Queen, Clubs;
///     King, Clubs;
///     King, Clubs
/// ).to_vec();
/// let result = eval.evaluate(&hand);
/// assert_eq!(
///     result,
///     Err(EvalError::CardsNotUnique(hand.clone()))
/// );
/// ```
///
/// A group of 5 or more cards can be evaulated for the combination that yields
/// the best possible poker hand, but four of less cards cannot be evaluated.
/// This will throw [`EvalError::InvalidHandSize`]:
///
/// ```
/// use poker::{cards, Evaluator, EvalError};
/// let eval = Evaluator::new();
/// let four_cards = cards!(
///     Two, Clubs;
///     Ten, Hearts;
///     Seven, Hearts;
///     Eight, Spades;
/// ).to_vec();
/// let result = eval.evaluate(&four_cards);
/// assert_eq!(
///     result,
///     Err(EvalError::InvalidHandSize(4)),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// This variant is used when the cards to be evaluated are not all unique.
    /// This captures the entire original hand, and the duplicates are
    /// calculated when reporting in [`Display`](std::fmt::Display) format.
    CardsNotUnique(Vec<Card>),
    /// This variant is used when the cards to be evaluated total to 4 or less.
    InvalidHandSize(usize),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CardsNotUnique(cards) => {
                let dups: String = cards
                    .iter()
                    .counts()
                    .into_iter()
                    .filter_map(|(card, count)| {
                        if count > 1 {
                            Some(card.rank_suit_string())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(
                    f,
                    "Cannot evaluate a poker hand with a set of cards that are not unique. Cards \
                     duplicated at least once: {}",
                    dups
                )
            }
            Self::InvalidHandSize(size) => write!(
                f,
                "Cannot evaluate a poker hand with a set of less than 5 cards. Number of cards \
                 received: {}",
                size
            ),
        }
    }
}

impl Error for EvalError {}
