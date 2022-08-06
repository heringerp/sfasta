mod sequence;
mod parser;
mod sequence_error;

pub use crate::sequence::dna_sequence;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
