use std::ops::Index;
use crate::sequence::Sequence;
use crate::sequence_error::SequenceError;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum DNucleotide {
    A,
    C,
    G,
    T
}

impl DNucleotide {
    pub fn from_char(c: char) -> Result<Self, SequenceError> {
        match c {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err(SequenceError::InvalidSymbol)
        }
    }

    pub fn get_complement(&self) -> Self {
        match self {
            Self::A => Self::T,
            Self::C => Self::G,
            Self::G => Self::C,
            Self::T => Self::A
        }
    }
}

#[derive(Debug)]
pub struct DNASequence {
    seq: Box<Vec<DNucleotide>>
}

impl DNASequence {
    pub fn get_reverse_complement(&self) -> Self {
        let sequence: Vec<_> = self.seq.iter().map(|x| x.get_complement()).rev().collect();
        DNASequence{ seq: Box::new(sequence) }
    }
}

impl Sequence for DNASequence {
    fn create(seq: &str) -> Result<Self, SequenceError> {
        let sequence: Result<Vec<_>,_> = seq.chars().map(|x| DNucleotide::from_char(x)).collect();
        Ok( DNASequence{ seq: Box::new(sequence?) } )
    }

    fn len(&self) -> usize {
        self.seq.len()
    }
}

impl PartialEq for DNASequence {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }

}

impl Eq for DNASequence {}

impl Index<usize> for DNASequence {
    type Output = DNucleotide;

    fn index(&self, index: usize) -> &Self::Output {
        &self.seq[index]
    }
}

impl IntoIterator for DNASequence {
    type Item = DNucleotide;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.seq.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn create_sequence() -> DNASequence {
        let nucleotides = vec![DNucleotide::A, DNucleotide::T, DNucleotide::C, DNucleotide::G];
        DNASequence{ seq: Box::new(nucleotides) }
    }

    #[test]
    fn equality() {
        let seq1 = create_sequence();
        let seq2 = create_sequence();
        assert_eq!(seq1, seq2);
    }

    #[test]
    fn create_sequence_from_string() {
        let direct_seq = create_sequence();
        let string_seq = DNASequence::create("ATCG").unwrap();
        assert_eq!(direct_seq, string_seq);
    }

    #[test]
    fn reverse_complement() {
        let orig = DNASequence::create("ATCG").unwrap();
        let rev_comp = DNASequence::create("CGAT").unwrap();
        assert_eq!(orig.get_reverse_complement(), rev_comp);
    }
}
