//! # Poker
//!
//! `poker` is a crate for efficient poker hand evaluation ported into Rust from
//! the [`treys`] Python package. This packages introduces the algorithms that
//! are vital for speedy evaluation, which have been added on to or made more
//! idiomatic for Rust where appropriate.
//!
//! ```
//! # fn main() {
//! #     if let Err(_) = run() { ::std::process::exit(1) }
//! # }
//! #
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use poker::{cards, Card, EvalClass, Evaluator, Rank};
//!
//! // Create a reusable evaluator
//! let eval = Evaluator::new();
//!
//! // Parse a `Vec` of cards from a str
//! let cards: Vec<Card> = cards!("Ks Js Ts Qs As").try_collect()?;
//!
//! // Evaluate the hand
//! let hand = eval.evaluate(cards)?;
//!
//! assert!(matches!(
//!     hand.class(),
//!     EvalClass::StraightFlush {
//!         high_rank: Rank::Ace
//!     }
//! ));
//! assert!(hand.is_royal_flush());
//! # Ok(())
//! # }
//! ```
//! [`treys`]: https://github.com/ihendley/treys

#![forbid(unsafe_code, missing_docs)]
#![doc(html_root_url = "https://docs.rs/poker/0.2")]

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
