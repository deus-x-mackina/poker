use std::{
    convert::TryFrom,
    fmt::{self, Write},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// An enumeration type for representing the thirteen card ranks, from two to
/// ace.
///
/// `Rank` has implemented [`Ord`] such that:
/// - 2 < 3 < 4 < 5 < 6 < 7 < 8 < 9 < T < J < Q < K < A
pub enum Rank {
    /// The rank of two, also called a deuce.
    Two,
    /// The rank of three, also called a trey.
    Three,
    /// The rank of four.
    Four,
    /// The rank of five.
    Five,
    /// The rank of six.
    Six,
    /// The rank of seven.
    Seven,
    /// The rank of eight.
    Eight,
    /// The rank of nine.
    Nine,
    /// The rank of ten.
    Ten,
    /// The rank of jack.
    Jack,
    /// The rank of queen.
    Queen,
    /// The rank of king.
    King,
    /// The rank of ace.
    Ace,
}

impl Rank {
    /// Get a textual representation of the rank. The character returned is the
    /// same character expected when parsing a rank from strings.
    ///
    /// # Example
    ///
    /// ```
    /// use poker::Rank;
    /// let ten = Rank::Ten;
    /// assert_eq!(ten.as_char(), 'T');
    /// ```
    pub const fn as_char(self) -> char {
        use Rank::*;
        match self {
            Ace => 'A',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Ten => 'T',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
        }
    }

    /// Get the integer representation of the rank, a number from 0 to 12
    /// inclusive.
    pub(super) const fn as_i32(self) -> i32 {
        use Rank::*;
        match self {
            Two => 0,
            Three => 1,
            Four => 2,
            Five => 3,
            Six => 4,
            Seven => 5,
            Eight => 6,
            Nine => 7,
            Ten => 8,
            Jack => 9,
            Queen => 10,
            King => 11,
            Ace => 12,
        }
    }

    /// Create a rank from its integer representation. As this function is
    /// private, be sure to only pass in 0 through 12 inclusive.
    pub(super) const fn from_i32(val: i32) -> Self {
        use Rank::*;
        match val {
            0 => Two,
            1 => Three,
            2 => Four,
            3 => Five,
            4 => Six,
            5 => Seven,
            6 => Eight,
            7 => Nine,
            8 => Ten,
            9 => Jack,
            10 => Queen,
            11 => King,
            // FIXME: Really, this should be 12 => Ace, _ => unreachable!() but you can't panic
            // in const functions in stable Rust yet.
            _ => Ace,
        }
    }

    /// Get the string name of this rank. Used for printing hands such as "ace
    /// high".
    pub(crate) const fn as_str_name(self) -> &'static str {
        use Rank::*;
        match self {
            Two => "two",
            Three => "three",
            Four => "four",
            Five => "five",
            Six => "six",
            Seven => "seven",
            Eight => "eight",
            Nine => "nine",
            Ten => "ten",
            Jack => "jack",
            Queen => "queen",
            King => "king",
            Ace => "ace",
        }
    }

    /// Get the plural string name of this rank. Used for printing hands such as
    /// "pair of aces".
    pub(crate) const fn as_str_name_plural(self) -> &'static str {
        use Rank::*;
        match self {
            Two => "twos",
            Three => "threes",
            Four => "fours",
            Five => "fives",
            Six => "sixes",
            Seven => "sevens",
            Eight => "eights",
            Nine => "nines",
            Ten => "tens",
            Jack => "jacks",
            Queen => "queens",
            King => "kings",
            Ace => "aces",
        }
    }
    
    pub(crate) const ALL_VARIANTS: &[Self] = &[
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];
}

impl TryFrom<char> for Rank {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Rank::*;
        match value {
            'A' => Ok(Ace),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            'T' => Ok(Ten),
            'J' => Ok(Jack),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            x => Err(x),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_char(self.as_char()) }
}
