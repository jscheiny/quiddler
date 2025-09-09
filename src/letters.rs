use crate::bit_set::BitSet;

pub type LetterSet = BitSet<u32>;
pub type LetterIndex = usize;
pub const ALPHABET_LENGTH: usize = 26;
pub const ALPHABET_START: usize = 'A' as usize;

pub fn get_letter_index(letter: char) -> LetterIndex {
    letter as LetterIndex - ALPHABET_START
}
