use crate::Rank;

/// Three-card metadata isn't an `enum`; the hand type can be figured out
/// by examining the `hand_rank` field.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Meta {
    pub hand_rank: i16,
    pub card_rank: Rank,
}