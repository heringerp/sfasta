use std::ops::Index;
use crate::sequence::{Sequence, Symbol};
use crate::sequence_error::SequenceError;

pub trait Complimentary {
    fn get_complement(&self) -> Self;
}

#[derive(Debug)]
pub struct ComplimentarySequence<T> where T: Complimentary + Symbol {
    seq: Vec<T>,
    header: Option<String>
}

impl<T> ComplimentarySequence<T> where T: Complimentary + Symbol {
    pub fn get_reverse_complement(&self) -> Self {
        let sequence: Vec<_> = self.seq.iter().map(|x| x.get_complement()).rev().collect();
        ComplimentarySequence{ seq: sequence, header: None }
    }
}

impl<T> Sequence for ComplimentarySequence<T> where T: Complimentary + Symbol{
    fn create(seq: &str, header: &str) -> Result<Self, SequenceError> {
        let sequence: Result<Vec<_>,_> = seq.chars().map(|x| T::from_char(x)).collect();
        Ok( ComplimentarySequence{ seq: sequence?, header: Some(header.to_string()) } )
    }

    fn len(&self) -> usize {
        self.seq.len()
    }
}

impl<T> PartialEq for ComplimentarySequence<T> where T: Complimentary + Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }

}

impl<T> Eq for ComplimentarySequence<T> where T: Complimentary + Symbol{}

impl<T> Index<usize> for ComplimentarySequence<T> where T: Complimentary + Symbol {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.seq[index]
    }
}

impl<T> IntoIterator for ComplimentarySequence<T> where T: Complimentary + Symbol{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.seq.into_iter()
    }
}

