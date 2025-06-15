//! This module is available under the non-default `static_lookup` feature and
//! offers similar functionality to the [`Evaluator`] type, but
//! [`evaluate_five`] comes as a free function. The main difference is that the
//! [`evaluate_five`] uses a static lookup table, built into the library.
//!
//! Because the `static` lookup table doesn't allocate any memory on the heap,
//! this module may become the foundation for providing `no_std` support in the
//! future.
//!
//! **Warning:** Enabling the `static_lookup` feature will greatly increase the
//! size of the resulting library.
//!
//! [`Evaluator`]: crate::Evaluator

use crate::{
    Card, Eval,
    error::EvalResult,
    evaluate::{
        evaluation::{self, Evaluation},
        poker_type::{FiveCard, ThreeCard},
    },
};

// This module includes the automatically generated code, fetched at build time.
mod statics {
    #[cfg(not(docsrs))]
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

    // Empty maps for docs.rs
    #[cfg(docsrs)]
    pub static FLUSH_LOOKUP_FIVE: ::phf::Map<
        i32,
        crate::evaluate::eval::Eval<crate::evaluate::poker_type::FiveCard>,
    > = ::phf::Map::new();

    #[cfg(docsrs)]
    pub static UNSUITED_LOOKUP_FIVE: ::phf::Map<
        i32,
        crate::evaluate::eval::Eval<crate::evaluate::poker_type::FiveCard>,
    > = ::phf::Map::new();

    #[cfg(docsrs)]
    pub static FLUSH_LOOKUP_THREE: ::phf::Map<
        i32,
        crate::evaluate::eval::Eval<crate::evaluate::poker_type::ThreeCard>,
    > = ::phf::Map::new();

    #[cfg(docsrs)]
    pub static UNSUITED_LOOKUP_THREE: ::phf::Map<
        i32,
        crate::evaluate::eval::Eval<crate::evaluate::poker_type::ThreeCard>,
    > = ::phf::Map::new();
}

use statics::{FLUSH_LOOKUP_FIVE, FLUSH_LOOKUP_THREE, UNSUITED_LOOKUP_FIVE, UNSUITED_LOOKUP_THREE};

// Helper struct for implementing Evaluation without having an actual struct
struct StaticEvaluator;

impl Evaluation for StaticEvaluator {
    type LookupFive = ::phf::Map<i32, Eval<FiveCard>>;
    type LookupThree = ::phf::Map<i32, Eval<ThreeCard>>;

    fn flush_lookup_five(&self) -> &Self::LookupFive { &FLUSH_LOOKUP_FIVE }

    fn unsuited_lookup_five(&self) -> &Self::LookupFive { &UNSUITED_LOOKUP_FIVE }

    fn flush_lookup_three(&self) -> &Self::LookupThree { &FLUSH_LOOKUP_THREE }

    fn unsuited_lookup_three(&self) -> &Self::LookupThree { &UNSUITED_LOOKUP_THREE }
}

/// Evaluate a hand using the static lookup table bundled with the library.
/// This function takes anything that implements `AsRef<[Card]>`, so owned or
/// borrowed slices of `Vec`s work fine here!
///
/// If you need to evaluate a hand in the context of a board (for example,
/// in Texas Holdem), you just need to combine both slices (such as with
/// [`box_cards!`]) and pass it to this method. See the exaples for
/// more.
///
/// # Errors
///
/// This function will fail if the total number of cards is less than five,
/// or if not all the cards passed in are unique. See
/// [`EvalError`](crate::EvalError) for more.
///
/// # Performance
///
/// Optimal performance is achieved with a set of 5, 6, or 7 cards. Hands
/// are evaulated using combinatorics to find the best 5-card
/// combination, so the more cards you pass to this method, the longer
/// it will take to evaluate.
///
/// # Example
///
/// ```
/// use poker::{Card, cards, evaluate::static_lookup};
///
/// const ROYAL_FLUSH: [Card; 5] = cards!(
///     Ten of Clubs,
///     Jack of Clubs,
///     Queen of Clubs,
///     King of Clubs,
///     Ace of Clubs,
/// );
/// let mut results = Vec::new();
/// // Pass a slice
/// results.push(static_lookup::evaluate_five(&ROYAL_FLUSH).expect("couldn't evaluate hand"));
/// // Pass an owned vector
/// results
///     .push(static_lookup::evaluate_five(ROYAL_FLUSH.to_vec()).expect("couldn't evaluate hand"));
/// assert!(results.into_iter().all(|result| result.is_royal_flush()));
/// ```
///
/// With a hand and a board:
///
/// ```
/// use poker::{
///     Card, box_cards, cards,
///     evaluate::{FiveCardHandClass, static_lookup},
/// };
///
/// let board: Vec<Card> = cards!("3c 5c As Jc Qh")
///     .try_collect()
///     .expect("couldn't parse cards");
/// let hand: Vec<Card> = cards!("Tc Ac").try_collect().expect("couldn't parse cards");
///
/// let result =
///     static_lookup::evaluate_five(box_cards!(board, hand)).expect("couldn't evaluate hand");
/// assert!(matches!(result.classify(), FiveCardHandClass::Flush { .. }));
/// ```
///
/// [`box_cards!`]: crate::box_cards
pub fn evaluate_five(cards: impl AsRef<[Card]>) -> EvalResult<FiveCard> {
    evaluation::evaluate_five(&StaticEvaluator, cards.as_ref())
}

/// Evaluates a three-card poker hand using pre-computed static lookup tables.
/// This function is available when the `static_lookup` feature is enabled.
pub fn evaluate_three(cards: impl AsRef<[Card]>) -> EvalResult<ThreeCard> {
    evaluation::evaluate_three(&StaticEvaluator, cards.as_ref())
}
