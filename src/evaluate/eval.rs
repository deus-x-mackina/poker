use std::marker::PhantomData;

use crate::{FiveCard, ThreeCard, evaluate::poker_type::PokerType};

/// Represents the successful results of evaluation a poker hand.
///
/// The generic type parameter `P` offers type safety and distinguishes results
/// from evaluating three (`Eval<ThreeCard>`) or five (`Eval<FiveCard>`) card
/// poker hands. `Eval` exposes similar functionality regardless of the value of
/// `P` (e.g., `classify()`), but some are unique to certain poker hand sizes
/// (e.g., `is_full_house()` for five-card hands).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Eval<P> {
    /// The rank of the poker hand.
    hand_rank: i16,
    /// Extra info about cards in the hand (e.g., pair of `Jacks` vs `Tens`).
    /// Can encode information for 1 or 2 ranks, as bitfields:
    /// 1 << 0 = 2
    /// 1 << 12 = Ace
    rank_flags: i16,
    /// Marks metadata as a three- or five-card poker hand.
    _poker_type: PhantomData<P>,
}

impl<P: PokerType> Eval<P> {
    pub(crate) const fn new(hand_rank: i16, rank_flags: i16) -> Self {
        Self {
            hand_rank,
            rank_flags,
            _poker_type: PhantomData,
        }
    }
}

impl<P> Eval<P> {
    /// The best possible five-card poker hand evaluation.
    pub const BEST_FIVE: Eval<FiveCard> = Eval::<FiveCard>::BEST;
    /// The best possible three-card poker hand evaluation.
    pub const BEST_THREE: Eval<ThreeCard> = Eval::<ThreeCard>::BEST;
    /// The worst possible five-card poker hand evaluation.
    pub const WORST_FIVE: Eval<FiveCard> = Eval::<FiveCard>::WORST;
    /// The worst possible three-card poker hand evaluation.
    pub const WORST_THREE: Eval<ThreeCard> = Eval::<ThreeCard>::WORST;

    pub(crate) const fn hand_rank(&self) -> i16 { self.hand_rank }

    pub(crate) const fn rank_flags(&self) -> i16 { self.rank_flags }
}
