use crate::card_data::{CardData, CardIndex};

pub struct DeckData {
    pub cards: Vec<CardData>,
}

impl DeckData {
    pub fn new() -> Self {
        DeckData {
            cards: vec![
                CardData::new("A", 2, 10),
                CardData::new("B", 8, 2),
                CardData::new("C", 8, 2),
                CardData::new("D", 5, 4),
                CardData::new("E", 2, 12),
                CardData::new("F", 6, 2),
                CardData::new("G", 6, 4),
                CardData::new("H", 7, 2),
                CardData::new("I", 2, 8),
                CardData::new("J", 13, 2),
                CardData::new("K", 8, 2),
                CardData::new("L", 3, 4),
                CardData::new("M", 5, 2),
                CardData::new("N", 5, 6),
                CardData::new("O", 2, 8),
                CardData::new("P", 6, 2),
                CardData::new("Q", 15, 2),
                CardData::new("R", 5, 6),
                CardData::new("S", 3, 4),
                CardData::new("T", 3, 6),
                CardData::new("U", 4, 6),
                CardData::new("V", 11, 2),
                CardData::new("W", 10, 2),
                CardData::new("X", 12, 2),
                CardData::new("Y", 4, 4),
                CardData::new("Z", 14, 2),
                CardData::new("ER", 7, 2),
                CardData::new("CL", 10, 2),
                CardData::new("IN", 7, 2),
                CardData::new("TH", 9, 2),
                CardData::new("QU", 9, 2),
            ],
        }
    }

    pub fn fmt(&self, index: CardIndex) -> String {
        format!("{}", self.cards[index])
    }
}
