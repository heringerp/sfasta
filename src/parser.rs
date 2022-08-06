use std::fs;

use crate::sequence::Sequence;
use crate::sequence::dna_sequence::{DNASequence, DNucleotide};
use crate::sequence_error::SequenceError;

pub fn read_dna(filename: &str) -> Result<Vec<DNASequence>, SequenceError> {
    let contents = fs::read_to_string(filename)?;
    parse_dna(&contents)
}

pub fn parse_dna(text: &str) -> Result<Vec<DNASequence>, SequenceError> {
    let lines: Vec<_> = text.lines().collect();
    let sequences: Result<Vec<_>, _> = lines.chunks(2).map(|x| DNASequence::create(x[1])).collect();
    Ok(sequences?)
}
