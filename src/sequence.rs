use std::ops::Index;
use crate::sequence_error::SequenceError;

pub mod dna_sequence;
mod complimentary_sequence;

trait Symbol: PartialEq {
    fn from_char(c: char) -> Result<Self, SequenceError> where Self: Sized;
}

pub trait Sequence: IntoIterator + Index<usize> + Eq + Sized {
    fn create(seq: &str, header: &str) -> Result<Self, SequenceError>;

    fn len(&self) -> usize;
}

