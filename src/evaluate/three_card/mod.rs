use lookup_table::LookupTable;

use crate::Card;

use super::evaluation::Evaluation;

#[doc(hidden)]
pub mod lookup_table;
mod meta;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Evaluator(LookupTable);

impl Evaluator {
    pub fn new() -> Self {
        Self(LookupTable::new())
    }
}