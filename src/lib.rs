//! # Poker
//!
//! `poker` is a crate for efficient poker hand evaluation supporting both
//! 5-card and 3-card poker hands. It is ported into Rust from the [`treys`]
//! Python package, utilizing fast lookup algorithms for speedy evaluation,
//! which have been adapted and made more idiomatic for Rust where appropriate.
//!
//! ## 5-Card Poker Example
//!
//! ```
//! # fn main() {
//! #     if run().is_err() { std::process::exit(1); }
//! # }
//! #
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use poker::{Card, Evaluator, Rank, cards, evaluate::FiveCardHandClass};
//!
//! // Create a reusable evaluator
//! let eval = Evaluator::new();
//!
//! // Parse a `Vec` of cards from a str
//! let royal_flush_cards: Vec<Card> = cards!("Ks Js Ts Qs As").try_collect()?;
//!
//! // Evaluate the hand
//! let royal_flush_eval = eval.evaluate_five(royal_flush_cards)?;
//!
//! assert!(matches!(
//!     royal_flush_eval.classify(),
//!     FiveCardHandClass::StraightFlush { rank: Rank::Ace }
//! ));
//! assert!(royal_flush_eval.is_royal_flush());
//!
//! // Compare hands
//! let pair_cards: Vec<Card> = cards!("3c 4h Td 3h Kd").try_collect()?;
//! let pair_eval = eval.evaluate_five(pair_cards)?;
//! assert!(royal_flush_eval.is_better_than(pair_eval));
//! # Ok(())
//! # }
//! ```
//!
//! ## 3-Card Poker Example
//!
//! ```
//! # fn main() {
//! #     if run().is_err() { std::process::exit(1); }
//! # }
//! #
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use poker::{Card, Evaluator, Rank, cards};
//!
//! // Create a reusable evaluator
//! let eval = Evaluator::new();
//!
//! // Three-card straight flush
//! let straight_flush: Vec<Card> = cards!("Ah Kh Qh").try_collect()?;
//! let sf_eval = eval.evaluate_three(straight_flush)?;
//!
//! assert!(sf_eval.is_straight_flush());
//! assert_eq!(sf_eval.straight_flush(), Some(Rank::Ace));
//!
//! // Compare with three of a kind (straight flush beats three of a kind)
//! let trips: Vec<Card> = cards!("As Ad Ac").try_collect()?;
//! let trips_eval = eval.evaluate_three(trips)?;
//! assert!(sf_eval.is_better_than(trips_eval));
//! # Ok(())
//! # }
//! ```
//!
//!
//! The [`Evaluator`] does not expose any mutable methods, so it's perfectly
//! safe to wrap it into an [`Arc`](std::sync::Arc) and share it between
//! multiple threads.
//!
//! ```
//! # fn main() {
//! #     if run().is_err() { std::process::exit(1); }
//! # }
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use std::{cmp, sync::Arc, thread};
//!
//! use poker::{Eval, Evaluator, FiveCard, deck};
//!
//! let shared_evaluator = Arc::new(Evaluator::new());
//! let mut handles = vec![];
//! for _ in 0..10 {
//!     let evaluator = Arc::clone(&shared_evaluator);
//!     handles.push(thread::spawn(move || {
//!         let deck = deck::shuffled();
//!         let hand = &deck[..5];
//!         evaluator
//!             .evaluate_five(hand)
//!             .unwrap_or(Eval::<FiveCard>::WORST_FIVE)
//!     }));
//! }
//!
//! let max = handles
//!     .into_iter()
//!     .filter_map(|handle| handle.join().ok())
//!     .fold(Eval::<FiveCard>::WORST_FIVE, cmp::max);
//!
//! println!("{}", max);
//! # Ok(())
//! # }
//! ```
//! [`treys`]: https://github.com/ihendley/treys

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/poker/0.7")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod card;
mod constants;
pub mod deck;
pub mod error;
pub mod evaluate;

#[doc(inline)]
pub use card::{Card, Rank, Suit};
#[doc(inline)]
pub use error::{EvalError, EvalResult, ParseCardError};
#[doc(inline)]
pub use evaluate::{
    eval::Eval,
    evaluator::Evaluator,
    poker_type::{FiveCard, ThreeCard},
};
