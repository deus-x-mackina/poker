/// A utility macro for creating a single card.
///
/// # Examples
///
/// Use [`card!`] with a capitalized rank and suit in order to get a
/// `const`-friendly card. The capitalization is because the names fill in as
/// enum variant names for [`Rank`] and [`Suit`]
///
/// ```
/// use poker::{card, Card, Rank, Suit};
/// const ACE_OF_SPADES: Card = card!(Ace, Spades);
/// assert_eq!(ACE_OF_SPADES, Card::new(Rank::Ace, Suit::Spades));
/// ```
///
/// Alternatively, you can separate the rank and suit with `of` for more
/// readability.
///
/// ```
/// use poker::{card, Card, Rank, Suit};
/// const ACE_OF_SPADES: Card = card!(Ace of Spades);
/// assert_eq!(ACE_OF_SPADES, Card::new(Rank::Ace, Suit::Spades));
/// ```
///
/// Finally, you can pass in a string expression, which will result in a call to
/// `parse()`. This, then, results in a [`Result`] and will fail if the rank is
/// not one of "23456789TJQKA" and the suit is not one of "chsd". This is
/// case-sensitive!
///
/// ```
/// use poker::{card, Card, Rank, Suit};
/// let ace_of_spades = card!("As").expect("couldn't parse card");
/// assert_eq!(ace_of_spades, Card::new(Rank::Ace, Suit::Spades));
/// ```
///
/// [`Rank`]: crate::Rank
/// [`Suit`]: crate::Suit
#[macro_export]
macro_rules! card {
    ($rank:ident, $suit:ident) => {
        $crate::Card::new($crate::Rank::$rank, $crate::Suit::$suit)
    };
    ($rank:ident of $suit:ident) => {
        $crate::card!($rank, $suit);
    };
    ($card_string:expr) => {
        $card_string.parse::<$crate::card::Card>()
    };
}

/// A utility macro for creating multiple cards.
///
/// # Examples
///
/// You can specify multiple cards by entering a comma-separated pair of
/// capitalized rank and suit, and with each pair being separated by a
/// semicolon. The reason for the zapitalized names is because the macro inlines
/// these as enum variants of [`Rank`] and [`Suit`]. This is `const`-friendly
/// and results in an array type.
///
/// ```
/// use poker::{cards, Card, Rank, Suit};
///
/// const KING_AND_QUEEN: [Card; 2] = cards!(
///     King, Hearts;
///     Queen, Hearts; // trailing semicolon supported
/// );
/// assert_eq!(
///     KING_AND_QUEEN,
///     [
///         Card::new(Rank::King, Suit::Hearts),
///         Card::new(Rank::Queen, Suit::Hearts)
///     ]
/// );
/// ```
///
/// Secondly, you can specify multiple cards by separating rank and suit with
/// "of" and then separating each card with a comma. This may be more readable.
/// As in the previous example, this is `const`-friendly and returns an array.
///
/// ```
/// use poker::{cards, Card, Rank, Suit};
///
/// const KING_AND_QUEEN: [Card; 2] = cards!(
///     King of Hearts,
///     Queen of Hearts, // trailing comma supported
/// );
/// assert_eq!(
///     KING_AND_QUEEN,
///     [
///         Card::new(Rank::King, Suit::Hearts),
///         Card::new(Rank::Queen, Suit::Hearts)
///     ]
/// );
/// ```
///
/// As a third option, you can pass in a comma separated list of strings
/// representing cards, where each string is parsed individually. This returns
/// an iterator that will parse each string, yielding `Ok(Card)` on success, and
/// `Err(ParseCardError)` on failure. This macro calls [`Card::parse_to_iter`]
/// internally, so you can use [`try_collect`] on the iterator.
///
/// ```
/// use poker::{cards, Card, Rank, Suit};
///
/// // Collect into a `Vec<Card>` by using `collect::<Result<_, _>>()` on an iterator that
/// // yields `Result` types.
/// let king_and_queen: Vec<_> = cards!("Kh", "Qh")
///     .try_collect()
///     .expect("couldn't parse cards");
///
/// assert_eq!(
///     king_and_queen,
///     [
///         Card::new(Rank::King, Suit::Hearts),
///         Card::new(Rank::Queen, Suit::Hearts)
///     ]
/// );
/// ```
///
/// Finally, you can pass in a single string expression of cards, separated by
/// whitespace. This also returns an iterator over each card expression,
/// yielding a `Result`. You can also call [`try_collect`] in this case.
///
/// ```
/// use poker::{cards, Card, Rank, Suit};
///
/// let king_and_queen: Vec<_> = cards!("Kh Qh").try_collect().expect("couldn't parse cards");
///
/// assert_eq!(
///     king_and_queen,
///     [
///         Card::new(Rank::King, Suit::Hearts),
///         Card::new(Rank::Queen, Suit::Hearts)
///     ]
/// );
/// ```
///
/// [`Rank`]: crate::Rank
/// [`Suit`]: crate::Suit
/// [`Card::parse_to_iter`]: crate::Card::parse_to_iter
/// [`try_collect`]: crate::card::ParseToIter::try_collect
#[macro_export]
macro_rules! cards {
    ($($rank:ident, $suit:ident);+ $(;)?) => {
        [$($crate::card!($rank, $suit)),+]
    };
    ($($rank:ident of $suit:ident),+ $(,)?) => {
        $crate::cards!($($rank, $suit);+);
    };
    ($card_string:expr, $($card_strings:expr),+ $(,)?) => {
        $crate::Card::parse_to_iter(&[$card_string, $($card_strings),+])
    };
    ($card_strings_together:expr) => {
        $crate::Card::parse_to_iter($card_strings_together.split_whitespace())
    };
}

/// Use this macro to chain two or more slices of [`Card`] into a single boxed
/// slice of [`Card`]. This may be useful for bundling a hand and poker board
/// together for evaluation, as in Texas Holdem.
///
/// [`Card`]: crate::Card
#[macro_export]
macro_rules! box_cards {
    ($card_slice:expr, $($card_slices:expr),+ $(,)?) => {
        $card_slice.iter().cloned()
            $(
                .chain($card_slices.iter().cloned())
            )+
            .collect::<Box<[$crate::Card]>>()
    };
}

#[cfg(test)]
mod tests {
    use crate::{box_cards, card, cards};
    #[test]
    fn box_cards_works() {
        let cards1 = &cards!(Ace of Clubs);
        let cards2 = &cards!(Two of Clubs);
        let cards3 = &cards!(Three of Clubs);
        let cards = box_cards!(cards1, cards2, cards3);
        assert_eq!(
            &*cards,
            [
                card!(Ace of Clubs),
                card!(Two of Clubs),
                card!(Three of Clubs)
            ]
        );
    }
}
