/// Marker trait for different poker hand types (three-card vs five-card).
///
/// Note that this trait is not intended to be implemented by users of this
/// library.
pub trait PokerType: private::Sealed {}

/// Marker type for five-card poker hands.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FiveCard;

impl PokerType for FiveCard {}

/// Marker type for three-card poker hands.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ThreeCard;

impl PokerType for ThreeCard {}

mod private {
    pub trait Sealed {}

    impl Sealed for super::FiveCard {}
    impl Sealed for super::ThreeCard {}
}
