use crate::sequence::Symbol;
use crate::sequence::complimentary_sequence::{Complimentary, ComplimentarySequence};
use crate::sequence_error::SequenceError;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum DNucleotide {
    A,
    C,
    G,
    T
}

impl Symbol for DNucleotide {
    fn from_char(c: char) -> Result<Self, SequenceError> {
        match c {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err(SequenceError::InvalidSymbol)
        }
    }
}

impl Complimentary for DNucleotide {
    fn get_complement(&self) -> Self {
        match self {
            Self::A => Self::T,
            Self::C => Self::G,
            Self::G => Self::C,
            Self::T => Self::A
        }
    }
}

pub type DNASequence = ComplimentarySequence<DNucleotide>;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Sequence;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn create_sequence() -> DNASequence {
        let nucleotides = vec![DNucleotide::A, DNucleotide::T, DNucleotide::C, DNucleotide::G];
        DNASequence{ seq: nucleotides,  }
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
        let string_seq = DNASequence::create("ATCG", "").unwrap();
        assert_eq!(direct_seq, string_seq);
    }

    #[test]
    fn reverse_complement() {
        let orig = DNASequence::create("ATCG", "").unwrap();
        let rev_comp = DNASequence::create("CGAT", "").unwrap();
        assert_eq!(orig.get_reverse_complement(), rev_comp);
    }
}
