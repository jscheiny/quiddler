use std::fmt::Display;
pub type CardIndex = usize;

pub struct CardData {
    pub letters: String,
    pub score: i32,
    pub count: u32,
}

impl CardData {
    pub fn new(letters: &str, score: i32, count: u32) -> Self {
        let letters = String::from(letters);
        CardData {
            letters,
            score,
            count,
        }
    }
}

impl Display for CardData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.letters, self.score)
    }
}
