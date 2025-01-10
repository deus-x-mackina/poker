pub trait PokerType {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FiveCard;

impl PokerType for FiveCard {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ThreeCard;

impl PokerType for ThreeCard {}
