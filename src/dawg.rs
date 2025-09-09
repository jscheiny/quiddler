use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::letters::{ALPHABET_LENGTH, LetterIndex, LetterSet, get_letter_index};

pub struct Dawg {
    nodes: Vec<DawgNode>,
}

pub struct DawgNode {
    pub is_end_of_word: bool,
    pub children: LetterSet,
    children_indices: Vec<usize>,
}

impl Dawg {
    pub fn read(
        path: &str,
        min_word_length: usize,
        max_word_length: usize,
    ) -> Result<Dawg, Box<dyn Error>> {
        let mut dawg = Dawg::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut count = 0;
        for line in reader.lines() {
            let word = line?;
            if word.len() >= min_word_length && word.len() <= max_word_length {
                dawg.add(word.as_str());
                count += 1;
            }
        }

        println!("Read in {} words", count);

        Ok(dawg)
    }

    pub fn new() -> Dawg {
        let root = DawgNode::new();
        Dawg { nodes: vec![root] }
    }

    pub fn node(&self, index: usize) -> &DawgNode {
        &self.nodes[index]
    }

    pub fn add(&mut self, word: &str) {
        self.add_impl(word, 0);
    }

    fn add_impl(&mut self, suffix: &str, node_index: usize) {
        if suffix.is_empty() {
            self.nodes[node_index].is_end_of_word = true;
            return;
        }

        let letter: char = suffix.chars().next().unwrap();
        let letter_index = get_letter_index(letter);

        let nodes_len = self.nodes.len();
        let node = self.node(node_index);

        if node.has_child(letter_index) {
            self.add_impl(&suffix[1..], node.child(letter_index));
        } else {
            let node = &mut self.nodes[node_index];
            let new_node = DawgNode::new();
            node.put_child(letter_index, nodes_len);
            self.nodes.push(new_node);
            self.add_impl(&suffix[1..], nodes_len);
        }
    }
}

impl DawgNode {
    fn new() -> DawgNode {
        DawgNode {
            is_end_of_word: false,
            children_indices: vec![0; ALPHABET_LENGTH],
            children: LetterSet::new(),
        }
    }

    pub fn child(&self, letter_index: LetterIndex) -> usize {
        self.children_indices[letter_index]
    }

    pub fn has_child(&self, letter_index: LetterIndex) -> bool {
        self.children_indices[letter_index] != 0
    }

    fn put_child(&mut self, letter_index: LetterIndex, child: usize) {
        self.children_indices[letter_index] = child;
        self.children.add(letter_index);
    }
}

impl Default for DawgNode {
    fn default() -> Self {
        Self::new()
    }
}
