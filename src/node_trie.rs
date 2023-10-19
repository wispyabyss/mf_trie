use crate::letter::Letter;
use crate::trie::Trie;
use std::collections::HashMap;
use std::fmt::Debug;

/* TODO List:
1. Clean up todos in code below.
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
      For this second case, when we are traversing down to the final node, we can keep a top node
      object that will be the deepest node (x) that we find that does not satisfy either 1 or 2. If
      we find we can delete the word node (y), we can delete all the nodes between x and y.
    */
    fn delete(&mut self, word: &[L]) {
        if word.is_empty() { return }
        let mut node = self;
        let mut cannot_delete_node: Option<Node<L>> = None;
        for letter in &word[..word.len().saturating_sub(1)] {
            match node.letter_to_node_map.get_mut(letter) {
                None => { return }
                Some(node_box) => {
                    node = &mut (*node_box);
                }
            }
        }
        if let Some(last_letter) = word.last() {
            node.letter_to_is_word_map.entry(last_letter.clone())
                .and_modify(|e| *e = false)
                .or_insert(false);
        }
    }
}

impl<L: Letter> Node<L> {
    /* can_delete is used as a helper for the delete method.
    Say we are deleting the word mftrie from our trie. Our trie has the following structure, where
    () indicates a node and [k,v] indicates a map. The -> indicates which node nodebox refers to:

    ([m,is_word],[m,nodebox]) -> ([f,is_word],[f,nodebox]) -> ([t,is_word],[t,nodebox]) ->
    ([r,is_word],[r,nodebox]) -> ([i,is_word],[i,nodebox]) -> ([e,is_word],[e,nodebox])

    ([e,is_word],[e,nodebox]) can be deleted if:
        1. *nodebox is empty. This implies mftrie is not a subword for some larger word (ex: mftriex)
        2. [e,is_word] contains no other entries like [key,true]. This implies there are no other
        words like mftrix.
        2. [e,nodebox] contains no other keys. This implies there are no other words like mftrixy

    ([i,is_word],[i,nodebox]) can be deleted if:
        1. *nodebox is empty (ie, we deleted ([e,is_word],[e,nodebox])). This implies mftri is not
        a subword of some larger word.
        2. [i,is_word] contains no other entries like [key,true]. This implies there are no other
        words like mftrx.
        3. [i,nodebox] contains no other keys. This implies there are no other words like mftrxy.

     Note, the 3 conditions are the same for each node in the list. Note that this would be much
     easier if we could traverse up, but we don't have any up pointers. So, how can we do it?

     Here is how I envision the algorithm: As we iterate through the 6 nodes above, we keep track
     of the last node that breaks rules 2 and 3. Say this is ([f,is_word],[f,nodebox]) for example.
     Thus, ([t,is_word],[t,nodebox]) ... ([e,is_word],[e,nodebox]) satisfy 2 and 3.

     When we get to ([e,is_word],[e,nodebox]), we determine whether 1 is satisfied. If 1 is not
     satisfied, we can't delete anything. If 1 is satisfied, note how 1 will bubble up and be
     satisfied for ([t,is_word],[t,nodebox]) ... ([e,is_word],[e,nodebox]). Thus, all the nodes
     beneath ([f,is_word],[f,nodebox]) can be deleted.

     And now we get to can_delete. I will probably rename this method, but it will essentially
     indicate whether the node satisfies 2 and 3.
     */
    fn can_delete(&self, letter: &L) -> bool {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::english_letter::EnglishLetter;
    use super::*;

    struct BaseTest {
        word_1: Vec<EnglishLetter>,
        word_2: Vec<EnglishLetter>,
        empty_trie: NodeTrie<EnglishLetter>,
        a_trie: NodeTrie<EnglishLetter>,
        mf_trie: NodeTrie<EnglishLetter>
    }

    impl BaseTest {
        fn new() -> Self {
            let word_1 = EnglishLetter::from_str("mftrie");
            let word_2 = EnglishLetter::from_str("mf");

            let empty_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
            let mut a_trie: NodeTrie<EnglishLetter> = NodeTrie::new();
            let mut mf_trie: NodeTrie<EnglishLetter> = NodeTrie::new();

            a_trie.add(&[EnglishLetter::A]);
            mf_trie.add(&word_1.as_slice());

            BaseTest {
                word_1,
                word_2,
                empty_trie,
                a_trie,
                mf_trie
            }
        }
    }

    #[test]
    fn test_new() {
        let base_test = BaseTest::new();

        assert!(!base_test.empty_trie.contains(&[EnglishLetter::A]));
    }

    #[test]
    fn test_add_one_letter_contains_one_letter() {
        let base_test = BaseTest::new();

        assert!(base_test.a_trie.contains(&[EnglishLetter::A]));
    }

    #[test]
    fn test_add_one_letter_does_not_contain_another_letter() {
        let base_test = BaseTest::new();

        assert!(!base_test.a_trie.contains(&[EnglishLetter::B]));
    }

    #[test]
    fn test_add_one_word_contains_one_word() {
        let base_test = BaseTest::new();

        assert!(base_test.mf_trie.contains(&base_test.word_1));
    }

    #[test]
    fn test_add_one_word_does_not_contain_shorter_word() {
        let base_test = BaseTest::new();

        assert!(!base_test.mf_trie.contains(&base_test.word_2));
    }

    #[test]
    fn test_add_one_word_does_not_contain_longer_word() {
        let base_test = BaseTest::new();
        let word = EnglishLetter::from_str("mftriex");

        assert!(!base_test.mf_trie.contains(&word.as_slice()));
    }

    #[test]
    fn test_add_two_words_contains_two_words() {
        let mut base_test = BaseTest::new();
        let word = EnglishLetter::from_str("mftriex");
        base_test.mf_trie.add(&word.as_slice());

        assert!(base_test.mf_trie.contains(&base_test.word_1.as_slice()));
        assert!(base_test.mf_trie.contains(&word.as_slice()));
    }

    #[test]
    fn test_add_word_delete_word_does_not_contain_word() {
        let mut base_test = BaseTest::new();

        assert!(base_test.a_trie.contains(&[EnglishLetter::A]));

        base_test.a_trie.delete(&[EnglishLetter::A]);

        assert!(!base_test.a_trie.contains(&[EnglishLetter::A]));
    }

    #[test]
    fn test_add_short_long_word_delete_short_contains_long() {
        let mut base_test = BaseTest::new();
        base_test.mf_trie.add(&base_test.word_2.as_slice());

        assert!(base_test.mf_trie.contains(&base_test.word_1.as_slice()));
        assert!(base_test.mf_trie.contains(&base_test.word_2.as_slice()));

        base_test.mf_trie.delete(&base_test.word_2.as_slice());

        assert!(base_test.mf_trie.contains(&base_test.word_1.as_slice()));
        assert!(!base_test.mf_trie.contains(&base_test.word_2.as_slice()));
    }

    #[test]
    fn test_add_short_long_word_delete_long_contains_short() {
        let mut base_test = BaseTest::new();
        base_test.mf_trie.add(&base_test.word_2.as_slice());

        assert!(base_test.mf_trie.contains(&base_test.word_1.as_slice()));
        assert!(base_test.mf_trie.contains(&base_test.word_2.as_slice()));

        base_test.mf_trie.delete(&base_test.word_1.as_slice());

        assert!(!base_test.mf_trie.contains(&base_test.word_1.as_slice()));
        assert!(base_test.mf_trie.contains(&base_test.word_2.as_slice()));
    }
}