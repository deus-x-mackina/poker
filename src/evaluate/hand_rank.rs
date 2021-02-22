use crate::evaluate::lookup_table;

/// The rank of a hand, ranging from 1 (best) to 7462 (worst).
/// This is not to be confused with a card rank! This number is mainly
/// used internally to compare hands easily using integer values (if one hand rank < other hand rank,
/// we know we have a *better* hand on the left!).
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct PokerHandRank(pub(crate) i16);

impl PokerHandRank {
    pub const BEST: Self = Self(1);
    pub const WORST: Self = Self(lookup_table::constants::WORST_HIGH_CARD as i16);

    /// Use this rather than Ord, because < meaning better can be confusing.
    #[inline]
    pub const fn is_better_than(self, other: Self) -> bool { self.0 < other.0 }
}
