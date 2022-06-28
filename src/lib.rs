//! # Poker
//!
//! `poker` is a crate for efficient poker hand evaluation ported into Rust from
//! the [`treys`] Python package. This packages introduces the algorithms that
//! are vital for speedy evaluation, which have been added on to or made more
//! idiomatic for Rust where appropriate.
//!
//! ```
//! # fn main() {
//! #     if run().is_err() { std::process::exit(1); }
//! # }
//! #
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use poker::{cards, Card, EvalClass, Evaluator, Rank};
//!
//! // Create a reusable evaluator
//! let eval = Evaluator::new();
//!
//! // Parse a `Vec` of cards from a str
//! let royal_flush_cards: Vec<Card> = cards!("Ks Js Ts Qs As").try_collect()?;
//!
//! // Evaluate the hand
//! let royal_flush_hand = eval.evaluate(royal_flush_cards)?;
//!
//! assert!(matches!(
//!     royal_flush_hand.class(),
//!     EvalClass::StraightFlush {
//!         high_rank: Rank::Ace
//!     }
//! ));
//! assert!(royal_flush_hand.is_royal_flush());
//!
//! // Compare hands
//! let pair_cards: Vec<Card> = cards!("3c 4h Td 3h Kd").try_collect()?;
//! let pair_hand = eval.evaluate(pair_cards)?;
//! assert!(royal_flush_hand.is_better_than(pair_hand));
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
//! use poker::{Card, Eval, Evaluator};
//!
//! let shared_evaluator = Arc::new(Evaluator::new());
//! let mut handles = vec![];
//! for _ in 0..10 {
//!     let evaluator = Arc::clone(&shared_evaluator);
//!     handles.push(thread::spawn(move || {
//!         let deck = Card::generate_shuffled_deck();
//!         let hand = &deck[..5];
//!         evaluator.evaluate(hand).unwrap_or(Eval::WORST)
//!     }));
//! }
//!
//! let max = handles
//!     .into_iter()
//!     .map(|handle| handle.join().unwrap())
//!     .fold(Eval::WORST, cmp::max);
//!
//! println!("{}", max);
//! # Ok(())
//! # }
//! ```
//! [`treys`]: https://github.com/ihendley/treys

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/poker/0.4")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod card;
mod constants;
pub mod error;
pub mod evaluate;
mod ext;

#[doc(inline)]
pub use card::{Card, Rank, Suit};
#[doc(inline)]
pub use error::{EvalError, ParseCardError};
#[doc(inline)]
pub use evaluate::{Eval, EvalClass, Evaluator};
