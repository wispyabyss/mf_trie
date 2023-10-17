use crate::letter::Letter;
use std::fmt::Debug;

pub trait Trie<L: Letter>: Debug {
    fn new() -> Self;
    fn add(&mut self, word: &[L]);
    fn contains(&self, word: &[L]) -> bool;
    fn delete(&mut self, word: &[L]);
}