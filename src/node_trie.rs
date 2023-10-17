use crate::letter::Letter;
use crate::trie::Trie;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<L: Letter> {
    // TODO: Add a value that indicates if the current node is the end of a word
    // This helps for the case when MFTRIE is a word, but MF is not
    // This will also help with the delete method, if we want a simple implementation - if we
    // can traverse to the end of the word, just update the value. More space efficient method
    // would delete the node if it had no children, and the list of above nodes that have is_word = false
    // until we reach a node where is_word = true.
    children: HashMap<L, Box<Node<L>>>
}

#[derive(Debug)]
pub struct NodeTrie<L: Letter> {
    roots: HashMap<L, Box<Node<L>>>
}

impl<L: Letter> Trie<L> for NodeTrie<L> {
    fn new() -> Self {
        Self {
            roots: HashMap::new()
        }
    }

    fn add(&mut self, word: &[L]) {
        let mut letter_hash_map = &mut self.roots;
        for letter in word {
            let new_node_box: Box<Node<L>> = Box::new(Node {
                children: HashMap::new()
            });

            let node_box = letter_hash_map
                .entry(letter.clone())
                .or_insert(new_node_box);

            letter_hash_map = &mut node_box.children;
        }
    }

    fn contains(&self, word: &[L]) -> bool {
        let mut letter_hash_map = &self.roots;
        for letter in word {
            match letter_hash_map.get(letter) {
                None => { return false }
                Some(node_box) => {
                    letter_hash_map = &(*node_box).children;
                }
            }
        }
        return true;
    }

    fn delete(&mut self, word: &[L]) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::english_letter::EnglishLetter;
    use super::*;

    #[test]
    fn test_new() {
        let empty_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word = [EnglishLetter::A];
        assert!(!empty_trie.contains(&word));
    }

    #[test]
    fn test_add_one_letter_contains_one_letter() {
        let mut a_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        a_trie.add(&[EnglishLetter::A]);
        assert!(a_trie.contains(&[EnglishLetter::A]));
    }

    #[test]
    fn test_add_one_letter_does_not_contain_another_letter() {
        let mut a_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        a_trie.add(&[EnglishLetter::A]);
        assert!(!a_trie.contains(&[EnglishLetter::B]));
    }

    #[test]
    fn test_add_one_word_contains_one_word() {
        let mut a_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word = [
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T,
            EnglishLetter::R,
            EnglishLetter::I,
            EnglishLetter::E
        ];
        a_trie.add(&word);
        assert!(a_trie.contains(&word));
    }

    #[test]
    fn test_add_one_word_does_not_contain_another_word() {
        let mut a_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word_1 = [
            EnglishLetter::M,
            EnglishLetter::F
        ];
        let word_2 = [
            EnglishLetter::T,
            EnglishLetter::R
        ];
        a_trie.add(&word_1);
        assert!(!a_trie.contains(&word_2));
    }

    #[test]
    fn test_add_one_word_does_not_contain_longer_word() {
        let mut a_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word_1 = [
            EnglishLetter::M,
            EnglishLetter::F
        ];
        let word_2 = [
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T
        ];
        a_trie.add(&word_1);
        assert!(!a_trie.contains(&word_2));
    }

    #[test]
    fn test_add_two_words_contains_two_words() {
        let mut a_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word_1 = [
            EnglishLetter::M,
            EnglishLetter::F
        ];
        let word_2 = [
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T
        ];
        a_trie.add(&word_1);
        a_trie.add(&word_2);
        assert!(a_trie.contains(&word_1));
        assert!(a_trie.contains(&word_2));
    }
}