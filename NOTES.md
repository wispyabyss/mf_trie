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