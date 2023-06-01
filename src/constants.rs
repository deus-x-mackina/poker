//! Private constants used throughout the crate.

/// The first thirteen prime numbers in ascending order.
pub const PRIMES: [i32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// Represents a range from 0 to and including 12, used for representing card
/// ranks in some instances.
pub const INT_RANKS: std::ops::Range<i16> = 0..13;
pub const INT_RANKS_REV: [i16; 13] = [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
