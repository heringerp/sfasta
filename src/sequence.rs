use std::ops::Index;

pub mod dna_sequence;

trait Sequence: IntoIterator + Index<usize> + Eq {

    fn create(seq: &str) -> Result<Box<Self>, &str>;

    fn len(&self) -> usize;

}

