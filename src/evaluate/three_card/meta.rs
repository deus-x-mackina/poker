use crate::Rank;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Meta {
    pub hand_rank: i16,
    pub card_rank: Rank,
}