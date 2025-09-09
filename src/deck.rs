use crate::{card_data::CardIndex, deck_data::DeckData};
use rand::seq::SliceRandom;
use std::{cmp::min, rc::Rc};

pub struct Deck {
    start: usize,
    cards: Vec<CardIndex>,
}

impl Deck {
    pub fn new(data: Rc<DeckData>) -> Self {
        let mut rng = rand::rng();
        let mut cards = vec![];
        for (index, card) in data.cards.iter().enumerate() {
            for _ in 0..card.count {
                cards.push(index);
            }
        }

        cards.shuffle(&mut rng);
        Deck { start: 0, cards }
    }

    pub fn draw(&mut self, count: usize) -> &[CardIndex] {
        let max_index = min(self.start + count, self.cards.len());
        let result = &self.cards[self.start..max_index];
        self.start = max_index;
        result
    }

    pub fn len(&self) -> usize {
        self.cards.len() - self.start
    }
}
