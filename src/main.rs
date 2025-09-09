use crate::{dawg::Dawg, deck::Deck, deck_data::DeckData};
use std::rc::Rc;

mod bit_set;
mod card_data;
mod dawg;
mod deck;
mod deck_data;
mod letters;

fn main() {
    let deck_data = Rc::new(DeckData::new());
    let mut deck = Deck::new(deck_data.clone());
    while deck.len() > 0 {
        println!("Deck size = {}", deck.len());
        let cards = deck.draw(11);
        for (index, &card) in cards.iter().enumerate() {
            println!("{}) {}", index, deck_data.fmt(card));
        }
    }
    println!("Deck size = {}", deck.len());
    println!("Hello, world!");

    let dawg = Dawg::read("words.txt", 3, 10).unwrap();
}
