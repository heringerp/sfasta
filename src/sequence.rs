use std::ops::Index;
use crate::sequence_error::SequenceError;

pub mod dna_sequence;

pub trait Sequence: IntoIterator + Index<usize> + Eq + Sized {

    fn create(seq: &str) -> Result<Self, SequenceError>;

    fn len(&self) -> usize;

}

