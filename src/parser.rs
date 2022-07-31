use std::fs;
use crate::sequence::dna_sequence::{DNASequence, DNucleotide};

pub fn read_dna(filename: &str) -> Result<Vec<DNASequence>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    parse_dna(contents)
}

pub fn parse_dna(text: &str) -> Result<Vec<DNASequence>, Box<dyn Error>> {
}
