use std::ops::Index;
use crate::sequence::Sequence;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum DNucleotide {
    A,
    C,
    G,
    T
}

impl DNucleotide {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err("Invalid DNucleotide")
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
    seq: Vec<DNucleotide>
}

impl DNASequence {
    pub fn get_reverse_complement(&self) -> Self {
        let sequence: Vec<_> = self.seq.iter().map(|x| x.get_complement()).rev().collect();
        DNASequence{ seq: sequence }
    }
}

impl Sequence for DNASequence {
    fn create(seq: &str) -> Result<Box<Self>, &str> {
        let sequence: Result<Vec<_>,_> = seq.chars().map(|x| DNucleotide::from_char(x)).collect();
        Ok( Box::new(DNASequence{ seq: sequence? } ))
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

    #[test]
    fn equality() {
        let nucleotides = vec![DNucleotide::A, DNucleotide::T, DNucleotide::C, DNucleotide::G];
        let seq1 = DNASequence{ seq: nucleotides.clone() };
        let seq2 = DNASequence{ seq: nucleotides };
        assert_eq!(seq1, seq2);
    }

    #[test]
    fn create_sequence() {
        let nucleotides = vec![DNucleotide::A, DNucleotide::T, DNucleotide::C, DNucleotide::G];
        let direct_seq = DNASequence{ seq: nucleotides };
        let string_seq = *DNASequence::create("ATCG").unwrap();
        assert_eq!(direct_seq, string_seq);
    }

    #[test]
    fn reverse_complement() {
        let orig = *DNASequence::create("ATCG").unwrap();
        let rev_comp = *DNASequence::create("CGAT").unwrap();
        assert_eq!(orig.get_reverse_complement(), rev_comp);
    }
}
