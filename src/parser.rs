use std::fs;

use crate::sequence::Sequence;
use crate::sequence::dna_sequence::DNASequence;
use crate::sequence_error::SequenceError;

pub fn read_dna(filename: &str) -> Result<Vec<DNASequence>, SequenceError> {
    let contents = fs::read_to_string(filename)?;
    parse_dna(&contents)
}

pub fn parse_dna(text: &str) -> Result<Vec<DNASequence>, SequenceError> {
    let lines: Vec<_> = text.lines().collect();
    let trimmed_lines: Vec<_> = lines.into_iter()
        .filter(|x| x.trim() != "") // Filter empty lines
        .filter(|x| x.trim().chars().collect::<Vec<_>>()[0] != '#') // Filter comments
        .collect();
    let sequences: Result<Vec<_>, _> = trimmed_lines.chunks(2).map(|x| DNASequence::create(x[1], x[0])).collect();
    Ok(sequences?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text() {
        let sequences = parse_dna("").unwrap();
        assert_eq!(sequences, Vec::new());
    }

    #[test]
    fn ignore_empty_lines() {
        let text = "    \n\t\n  \n \t";
        let result = parse_dna(text).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn ignore_comments() {
        let line1 = "# comment";
        let line2 = "  \t# another comment";
        let result = parse_dna(&format!("{}\n{}", line1, line2)).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn one_entry() {
        let header = ">Simple header";
        let sequence = "A";
        let result = parse_dna(&format!("{}\n{}", header, sequence)).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn set_header() {
        let header = ">Simple header";
        let sequence = "A";
        let result = parse_dna(&format!("{}\n{}", header, sequence)).unwrap();
        assert_eq!(result[0].get_header().unwrap(), header);
    }
}
