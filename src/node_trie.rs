use crate::letter::Letter;
use crate::trie::Trie;
use std::collections::HashMap;
use std::fmt::Debug;

/* TODO List:
1. Clean up testing. Try and extract common code (such as trie creation) to a helper method.

2. Clean up todos in code below.
*/

pub type NodeTrie<L> = Node<L>;

#[derive(Debug)]
pub struct Node<L: Letter> {
    letter_to_node_map: HashMap<L, Box<Node<L>>>,
    letter_to_is_word_map: HashMap<L, bool>
}

impl<L: Letter> Trie<L> for Node<L> {
    fn new() -> Self {
        Self {
            letter_to_node_map: HashMap::new(),
            letter_to_is_word_map: HashMap::new()
        }
    }

    // TODO: Deduplicate. But, it seems to involve lifetimes, so leaving for now.
    fn add(&mut self, word: &[L]) {
        let mut node = self;
        for letter in &word[..word.len().saturating_sub(1)] {
            let new_node_box: Box<Node<L>> = Box::new(Node::new());
            let node_box = node.letter_to_node_map
                .entry(letter.clone())
                .or_insert(new_node_box);
            node = &mut (*node_box);
        }
        if let Some(last_letter) = word.last() {
            let new_node_box: Box<Node<L>> = Box::new(Node::new());
            let _ = node.letter_to_node_map
                .entry(last_letter.clone())
                .or_insert(new_node_box);

            node.letter_to_is_word_map.insert(last_letter.clone(), true);
        }
    }

    fn contains(&self, word: &[L]) -> bool {
        let mut node = self;
        for letter in &word[..word.len().saturating_sub(1)] {
            match node.letter_to_node_map.get(letter) {
                None => { return false }
                Some(node_box) => {
                    node = &(*node_box);
                }
            }
        }
        if let Some(last_letter) = word.last() {
            return node.letter_to_is_word_map.get(last_letter).unwrap_or(&false).clone()
        }
        false
    }

    fn delete(&mut self, word: &[L]) {
        if word.is_empty() { return }
        todo!();
        /* Delete description:
        1. EZ delete (first implementation) - just iterate to the final letter in the word, and
        update its value in the letter_to_is_word_map
        2. Hard delete.
          - Case One: final letter node has non empty letter_to_node_map. In this case, all we can
          do is update the value in letter_to_is_word_map
          - Case Two: final letter node has empty letter_to_node_map. In this case, we can delete
          the node. Then, we can start upward traversal, deleting every node, stopping only when:
            1. The letter_to_node_map contains a letter not matching the word
            2. The letter_to_is_word_map contains the word letter as a sub word
         */
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
        let mut mf_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word = [
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T,
            EnglishLetter::R,
            EnglishLetter::I,
            EnglishLetter::E
        ];
        mf_trie.add(&word);

        assert!(mf_trie.contains(&word));
    }

    #[test]
    fn test_add_one_word_does_not_contain_another_word() {
        let mut mf_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word_1 = [
            EnglishLetter::M,
            EnglishLetter::F
        ];
        let word_2 = [
            EnglishLetter::T,
            EnglishLetter::R
        ];
        mf_trie.add(&word_1);

        assert!(!mf_trie.contains(&word_2));
    }

    #[test]
    fn test_add_one_word_does_not_contain_longer_word() {
        let mut mf_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word_1 = [
            EnglishLetter::M,
            EnglishLetter::F
        ];
        let word_2 = [
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T
        ];
        mf_trie.add(&word_1);

        assert!(!mf_trie.contains(&word_2));
    }

    #[test]
    fn test_add_two_words_contains_two_words() {
        let mut mf_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word_1 = [
            EnglishLetter::M,
            EnglishLetter::F
        ];
        let word_2 = [
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T
        ];
        mf_trie.add(&word_1);
        mf_trie.add(&word_2);

        assert!(mf_trie.contains(&word_1));
        assert!(mf_trie.contains(&word_2));
    }

    #[test]
    fn test_add_long_word_does_not_contain_short_word() {
        let mut mf_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
        let word_1 = EnglishLetter::from_str("mftrie");
        let word_2 = EnglishLetter::from_str("mf");
        mf_trie.add(word_1.as_slice());

        assert!(mf_trie.contains(&word_1.as_slice()));
        assert!(!mf_trie.contains(&word_2.as_slice()));
    }
}