# notes

How I envision mf_trie will be structured:
1. We will have an MFTrie struct.
    - new(alphabet) - create a new MFTrie with the given alphabet
    - add(word) - add a word (alphabet sequence) to the trie structure
    - add(words) - add a set (is there iterable in rust?) of words to the trie
    - contains(word) - test if a word is contained within the trie
    - containsAll(words) - test if all of the words are contained within the trie
2. We will have an MFAlphabet trait.
    - equals(letter, letter) - test if two members of the alphabet are equal
    - letters() - return a list of letters within the alphabet
3. Standard alphabets
    - English alphabet (a,b,c,d,...)
    - UTF-8
    - Unicode

Ideas
- First implementation can be a custom struct implementation, with nodes and the like. 
- Next implementations can maybe add a hash map alongside the struct, for sub-sequences contained with the trie.
- Maybe, if optional functions can be added to a trait? we can add functions for like sequence equality (ie, check if 2 or 3 letter sequences are equal) that might be faster than iterating letter by letter
- Should def add bounds checks to the trie, ie word length is less than the maximum depth of the trie
- I like the node structure I have with node_trie. Some ideas, however, are:
  - For node trie, we can limit ourselves to 32 letters. Then, we can keep a 32 bit array for the letter_to_is_word_map. 
  - Also, for the letter_to_node_map, we can remove the hash map for a 32 bit vector.
  - In both cases, we can remove the hash map, because with 32 letter limit, we will know the exact memory location for each value related to the letter key
  - Also, we should try and keep the nodes in contiguous memory - somehow - and if we can keep the order the nodes appear in memory, all the better (ie, memory from left to right, left most is top level and starting letter, rightmost is bottom level ending letter). Might make delete difficult though.