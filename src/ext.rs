//! Private, useful type extension traits used throughout the crate.

use std::{collections::HashSet, hash::Hash};

/// A trait used to verify if all elements of a collection are unique from each
/// other.
pub(crate) trait AllUnique {
    /// Returns whether or not all elements are unique or not.
    fn all_unique(self) -> bool;
}

impl<T> AllUnique for T
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    #[inline]
    fn all_unique(self) -> bool {
        let mut iter = self.into_iter();
        let possible_size = {
            let (lower, upper) = iter.size_hint();
            upper.unwrap_or(lower)
        };
        let mut unique = HashSet::with_capacity(possible_size);
        iter.all(move |item| unique.insert(item))
    }
}
