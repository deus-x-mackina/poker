use std::{
    convert::TryFrom,
    fmt::{self, Write},
};

use variter::derive_var_iter;

derive_var_iter! {
    @impl_attr {
        #[doc(hidden)]
    }
    /// An enumeration type for representing the four card suits.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Suit {
        /// The suit of clubs.
        Clubs,
        /// The suit of hearts.
        Hearts,
        /// The suit of spades.
        Spades,
        /// The suit of diamonds.
        Diamonds,
    }
}

impl Suit {
    /// Get a Unicode representation of the suit, suitable for printing.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::Suit;
    /// assert_eq!(Suit::Hearts.as_pretty_char(), '♥',);
    /// assert_eq!(Suit::Diamonds.as_pretty_char(), '♦');
    /// assert_eq!(Suit::Clubs.as_pretty_char(), '♣');
    /// assert_eq!(Suit::Spades.as_pretty_char(), '♠');
    /// ```
    #[inline]
    pub const fn as_pretty_char(self) -> char {
        use Suit::*;
        match self {
            Spades => '\u{2660}',   // ♠
            Hearts => '\u{2665}',   // ♥
            Clubs => '\u{2663}',    // ♣
            Diamonds => '\u{2666}', // ♦
        }
    }

    /// Get a textual representation of the suit. The character returned is the
    /// same character expected when parsing a suit from strings.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::Suit;
    /// assert_eq!(Suit::Diamonds.as_char(), 'd');
    /// assert_eq!(Suit::Clubs.as_char(), 'c');
    /// assert_eq!(Suit::Spades.as_char(), 's');
    /// assert_eq!(Suit::Hearts.as_char(), 'h');
    /// ```
    #[inline]
    pub const fn as_char(self) -> char {
        use Suit::*;
        match self {
            Clubs => 'c',
            Hearts => 'h',
            Spades => 's',
            Diamonds => 'd',
        }
    }

    /// Get the integer representation of the suit, where:
    /// - 0b0001 <== spades
    /// - 0b0010 <== hearts,
    /// - 0b0100 <== diamonds,
    /// - 0b1000 <== clubs
    #[inline]
    pub(super) const fn as_i32(self) -> i32 {
        use Suit::*;
        match self {
            Clubs => 8,
            Hearts => 2,
            Spades => 1,
            Diamonds => 4,
        }
    }

    /// Create a suit from its integer representation. As this function is
    /// private, be sure to only pass in 1, 2, 4, or 8 to prevent the
    /// fallthrough case from executing.
    pub(super) const fn from_i32(val: i32) -> Self {
        use Suit::*;
        match val {
            0b1000 => Clubs,
            0b0100 => Diamonds,
            0b0010 => Hearts,
            0b0001 => Spades,
            // Should be unreachable
            #[cold] // Does this actually help? `unreachable!()` can't be used in a const-fn
            _ => Spades,
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = char;

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Suit::*;
        match value {
            's' => Ok(Spades),
            'c' => Ok(Clubs),
            'h' => Ok(Hearts),
            'd' => Ok(Diamonds),
            x => Err(x),
        }
    }
}

impl fmt::Display for Suit {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_char(self.as_pretty_char()) }
}
