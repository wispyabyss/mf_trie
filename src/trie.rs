use crate::letter::Letter;
use std::fmt::Debug;

pub trait Trie<L: Letter>: Debug {
    fn add(&self, word: &Vec<L>) -> Self;
    fn contains(&self, word: &Vec<L>) -> bool;
}