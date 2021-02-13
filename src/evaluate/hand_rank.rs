use crate::evaluate::lookup_table;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct PokerHandRank(pub(crate) i16);

impl PokerHandRank {
    pub const BEST: Self = Self(1);
    pub const WORST: Self = Self(lookup_table::constants::MAX_HIGH_CARD as i16);

    #[inline]
    pub const fn is_better_than(self, other: Self) -> bool { self.0 < other.0 }
}
