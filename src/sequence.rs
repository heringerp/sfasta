use std::ops::Index;

trait Sequence: IntoIterator + Index<u32> + Eq {

    fn create(seq: &str) -> Result<Box<Self>, &str>;

    fn len(&self) -> u32;

}

enum DNucleotide {
    A,
    C,
    G,
    T
}

impl DNucleotide {
    fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err("Invalid DNucleotide")
        }
    }
}

struct DNASequence {
    seq: Vec<DNucleotide>
}

impl Sequence for DNASequence {
    fn create(seq: &str) -> Result<Box<Self>, &str> {
        let sequence = seq.chars().map(|x| DNucleotide::from_char(x)?).collect::<Vec<DNucleotide>>();
        Ok( DNASequence{ seq: sequence } )
    }

    fn len(&self) -> u32 {
        self.seq.len()
    }
}

impl PartialEq for DNASequence {
    fn eq(&self, other: &Self) -> bool {

    }

}

impl Eq for DNASequence {}

impl Index<u32> for DNASequence {
    type Output = DNucleotide;

    fn index(&self, index: u32) -> &Self::Output {
        self.seq[index]
    }
}
