use std::fmt::Display;

use crate::card_data::{CardData, CardIndex};

#[derive(Clone)]
pub struct Word {
    text: String,
    cards: Vec<CardIndex>,
    score: i32,
}

impl Word {
    pub fn new() -> Word {
        Word {
            text: String::new(),
            cards: vec![],
            score: 0,
        }
    }

    pub fn push(&mut self, card_id: CardIndex, card: &CardData) {
        self.text.push_str(card.letters.as_str());
        self.cards.push(card_id);
        self.score += card.score;
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (score={} | length={} | cards={})",
            self.text,
            self.score,
            self.text.len(),
            self.cards.len()
        )
    }
}
