use std::ops::Index;
use crate::sequence::{Sequence, Symbol};
use crate::sequence_error::SequenceError;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AminoAcid {
    A, // Alanine
    C, // Cysteine
    D, // Aspartic Acid
    E, // Glutamic Adic
    F, // Phenylalanine
    G, // Glycine
    H, // Histidine
    I, // Isoleucine
    K, // Lysine
    L, // Leucine
    M, // Methionine
    N, // Asparagine
    P, // Proline
    Q, // Glutamine
    R, // Arginine
    S, // Serine
    T, // Threonine
    V, // Valine
    W, // Tryptophan
    Y, // Tyrosine
}

impl Symbol for AminoAcid {
    fn from_char(c: char) -> Result<Self, SequenceError> {
        match c {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            'F' => Ok(Self::F),
            'G' => Ok(Self::G),
            'H' => Ok(Self::H),
            'I' => Ok(Self::I),
            'K' => Ok(Self::K),
            'L' => Ok(Self::L),
            'M' => Ok(Self::M),
            'N' => Ok(Self::N),
            'P' => Ok(Self::P),
            'Q' => Ok(Self::Q),
            'R' => Ok(Self::R),
            'S' => Ok(Self::S),
            'T' => Ok(Self::T),
            'V' => Ok(Self::V),
            'W' => Ok(Self::W),
            'Y' => Ok(Self::Y),
            _ => Err(SequenceError::InvalidSymbol)
        }
    }
}

pub struct ProteinSequence {
    seq: Vec<AminoAcid>,
    header: Option<String>
}

impl Sequence for ProteinSequence {
    fn create(seq: &str, header: &str) -> Result<Self, SequenceError> {
        let sequence: Result<Vec<_>, _> = seq.chars().map(|x| AminoAcid::from_char(x)).collect();
        Ok( ProteinSequence{ seq: sequence?, header: Some(header.to_string()) } )
    }

    fn len(&self) -> usize {
        self.seq.len()
    }

    fn get_header(&self) -> Option<&str> {
        self.header.as_deref()
    }
}

impl PartialEq for ProteinSequence {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl Eq for ProteinSequence {}

impl Index<usize> for ProteinSequence {
    type Output = AminoAcid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.seq[index]
    }
}

impl IntoIterator for ProteinSequence {
    type Item = AminoAcid;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.seq.into_iter()
    }
}
