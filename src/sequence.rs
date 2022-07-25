use std::ops::Index;

trait Sequence: IntoIterator + Index<u32> + Eq {

    fn create(seq: &str) -> Result<Box<Self>, &str>;

    fn len(&self) -> u32;

}

struct DNASequence {
    seq: String
}

impl Sequence for DNASequence {

    fn create(seq: &str) -> Result<Box<Self>, &str> {
        Ok( DNASequence{ seq: seq.to_owned() } )
    }

    fn len(&self) -> u32 {
        self.seq.len()
    }
}
