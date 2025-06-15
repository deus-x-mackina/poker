//! Anything and everything about evaluating poker hands.
//!
//! Given the specific format in which [`Card`] instances are coded, the key is
//! figuring out how to leverage that data representation to quickly evaluate
//! poker hands. (See [the `card` module] for more information).
//!
//! The key is the [`Evaluator`](crate::Evaluator) structure, which dynamically
//! generates some internal [`HashMap`](std::collections::HashMap)s that it can
//! use to quickly look up poker hands by a hand's unique key, which is a
//! product of prime numbers that each [`Card`] instance codes.
//!
//! Because the [`Evaluator`](crate::Evaluator) must dynamically generate its
//! lookup tables at *runtime*, and the tables are decently sized, it is
//! recommended that:
//! - You instantiate an [`Evaluator`](crate::Evaluator) as soon as possible
//! - You avoid cloning the [`Evaluator`](crate::Evaluator)
//!
//! If there's going to be a performance bottleneck associated with this crate,
//! it will be making an [`Evaluator`](crate::Evaluator) from scratch. Even so,
//! in optimized benching, [`Evaluator::new`](crate::Evaluator::new) only takes
//! about 300 - 400 *microseconds* (there are 1 million microseconds in 1
//! second). Still, it is preferable to be conservative here. All
//! [`Evaluator`](crate::Evaluator) methods borrow `Self` immutably, so pass it
//! around as you see fit.
//!
//! [`Card`]: crate::Card
//! [the `card` module]: mod@crate::card

/// Core evaluation types and functionality.
pub mod eval;
mod evaluation;
pub mod evaluator;
mod five_card;
/// Marker traits for different poker variants (three-card, five-card).
pub mod poker_type;
#[cfg(feature = "static_lookup")]
pub mod static_lookup;
mod three_card;
mod utils;

pub use five_card::hand_class::FiveCardHandClass;
pub use three_card::hand_class::ThreeCardHandClass;
