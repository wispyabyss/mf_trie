use crate::letter::Letter;
use crate::trie::Trie;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<L: Letter> {
    value: L,
    children: HashMap<L, Box<Node<L>>>
}

#[derive(Debug)]
pub struct NodeTrie<L: Letter> {
    roots: HashMap<L, Box<Node<L>>>
}

impl<L: Letter> Trie<L> for NodeTrie<L> {
    fn add(&self, word: &Vec<L>) -> Self {
        todo!()
    }

    fn contains(&self, word: &Vec<L>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {}