use crate::{
    card_data::CardIndex, dawg::Dawg, deck::Deck, deck_data::DeckData, letters::get_letter_index,
};
use std::{collections::HashSet, rc::Rc};

mod bit_set;
mod card_data;
mod dawg;
mod deck;
mod deck_data;
mod letters;

fn main() {
    let deck_data = Rc::new(DeckData::new());
    let mut deck = Deck::new(deck_data.clone());
    let cards = deck.draw(11).iter().map(|x| *x).collect::<Vec<_>>();

    for &c in cards.iter() {
        println!("{}", deck_data.cards[c]);
    }

    let dawg = Dawg::read("words.txt", 3, 10).unwrap();
    find_longest_word(&dawg, &cards, &deck_data);
}

fn find_longest_word(dawg: &Dawg, cards: &Vec<CardIndex>, deck_data: &Rc<DeckData>) {
    let mut longest_word = String::from("");
    find_words_impl(dawg, 0, cards, deck_data, &String::from(""), &mut |word| {
        if word.len() > longest_word.len() {
            longest_word = word.clone();
        }
    });
    println!("{}", longest_word);
}

fn find_words_impl(
    dawg: &Dawg,
    node_index: usize,
    cards: &Vec<CardIndex>,
    deck_data: &Rc<DeckData>,
    prefix: &String,
    on_find_word: &mut impl FnMut(&String),
) {
    let node = dawg.node(node_index);

    if node.is_end_of_word {
        on_find_word(prefix);
    }

    let mut seen_cards = HashSet::new();
    for (index, &card_id) in cards.iter().enumerate() {
        if seen_cards.contains(&card_id) {
            continue;
        }

        let card = &deck_data.cards[card_id];
        let mut next_node = node;
        let mut next_node_index = node_index;
        let mut recur = true;
        let mut next_prefix = prefix.clone();
        for letter in card.letters.chars() {
            let letter_index = get_letter_index(letter);
            if next_node.has_child(letter_index) {
                next_node_index = next_node.child(letter_index);
                next_node = dawg.node(next_node_index);
                next_prefix.push(letter);
            } else {
                recur = false;
            }
        }

        if recur {
            let cards_subset = cards
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, card)| *card)
                .collect::<Vec<_>>();
            find_words_impl(
                dawg,
                next_node_index,
                &cards_subset,
                deck_data,
                &next_prefix,
                on_find_word,
            );
        }

        seen_cards.insert(card_id);
    }
}
