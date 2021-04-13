//! This module provides the [`Trie`] datastructure, a type of search tree.
#![warn(missing_docs, broken_intra_doc_links)]

use std::collections::HashMap;

/// The [Trie] datastructure.
///
/// The current implementation uses [`Node`]s to store the values inside the trie. Each [`Node`]
/// has a key and a value associated with it. The key is the last character of the value, and is
/// used as an index into the [`Node::children`] [`HashMap`]. The value contains the word which
/// would be found when traversing the trie from the root to that node.
///
/// To iterate over the words inside the trie, the user has two options: they can either iterate
/// over all the words in the trie, or they can iterate over the words with a given prefix.
///
/// [Trie]: https://en.wikipedia.org/wiki/Trie
///
/// # Examples
///
/// A basic example, showing how to insert a word into the trie, and how to iterate over the words
/// in the trie.
///
/// ```
/// use trie::Trie;
///
/// let mut trie = Trie::new();
///
/// trie.insert("Hello world");
/// trie.insert("Goedemorgen");
/// trie.insert("Hello sir");
///
/// // Iterate over all the words in the trie
/// for word in trie.words() {
///     println("Found word: {}", word);
/// }
///
/// // Iterate over all the words starting with 'Hello'
/// for word in trie.words_with_prefix("Hello") {
///     println("Found word: {}", word);
/// }
/// ```
#[derive(Debug)]
pub struct Trie {
    /// The root node inside the trie.
    ///
    /// This node serves no other purpose besides providing an easy way to access the nodes in the
    /// trie. The key and value shouldn't be read, as they have no meaning, and only serve as
    /// placeholders, to prevent us from having to store them inside an [`Option`], which wouldn't
    /// make sense as the key and value properties are mandatory on a [`Node`].
    root: Node,
}

impl Trie {
    /// Create an empty trie datastructure.
    pub fn new() -> Self {
        Self {
            root: Node::new(' ', String::new()),
        }
    }

    /// Inserts the `word` into the trie.
    ///
    /// If a part of the `word` is not yet present in the trie, that part is added. The already
    /// existing part of the `word` is unchanged.
    pub fn insert(&mut self, word: &str) {
        self.root.insert(word);
    }

    /// Deletes the `word` from the trie.
    ///
    /// Only the part that is not part of another word will be removed, if part of the `word` is a
    /// prefix of another word in the trie, that part will not be removed.
    pub fn delete(&mut self, word: &str) {
        self.root.delete(word);
    }

    /// Deletes the `word` from the trie after the `prefix`, leaving the `prefix` intact.
    pub fn delete_after_prefix(&mut self, prefix: &str, word: &str) {
        if let Some(head) = self.root.find_mut(prefix) {
            head.delete(word);
        }
    }

    /// Returns a reference to the [`Node`] containing the last character of the `word`.
    pub fn find(&self, word: &str) -> Option<&Node> {
        self.root.find(word)
    }

    /// Returns a mutable reference to the [`Node`] containing the last character of the `word`.
    fn find_mut(&mut self, word: &str) -> Option<&mut Node> {
        self.root.find_mut(word)
    }

    /// Returns an iterator over the words in the trie with the given prefix.
    pub fn words_with_prefix(&self, prefix: &str) -> TrieRead {
        let stack = if let Some(head) = self.find(prefix) {
            head.children.values().collect::<Vec<_>>()
        } else {
            vec![]
        };

        TrieRead { stack }
    }

    /// Returns an iterator over all the words in the trie.
    pub fn words(&self) -> TrieRead {
        TrieRead {
            stack: self.root.children.values().collect::<Vec<_>>(),
        }
    }
}

/// Iterator over the words in a [`Trie`]
///
/// This iterator is returned from the [`Trie::words_with_prefix`] function on a [`Trie`] and will
/// yield instances of [`String`].
pub struct TrieRead<'a> {
    /// Stack to keep track of which [`Node`]s we still need to visit while iterating over the
    /// words in the trie.
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for TrieRead<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        // Iterates over the words in the trie using depth-first search
        loop {
            // Get the next node to check
            if let Some(head) = self.stack.pop() {
                // Store the children on the stack, to be examined later
                for child in head.children.values() {
                    self.stack.push(child);
                }

                // If a node has no children, it is the end of a word, and we should return the
                // value, since that will contain a complete word. If the node does have children,
                // we don't return here, but simply continue looping until we either reach a node
                // containing a complete word, or we run out of nodes.
                if head.children.is_empty() {
                    return Some(&head.value);
                }
            } else {
                // We have looked through the entire trie, and are done iterating
                break;
            }
        }

        None
    }
}

/// A `Node` in a [`Trie`].
#[derive(Debug)]
pub struct Node {
    /// The last character of the word stored in the value.
    key: char,
    /// Contains the word which would be found when traversing the trie from the root to this node.
    value: String,
    /// The children, i.e. words which have `value` as a prefix.
    children: HashMap<char, Node>,
}

impl Node {
    /// Creates a new `Node` with the given key and value.
    fn new(key: char, value: String) -> Self {
        Self {
            key,
            value,
            children: HashMap::new(),
        }
    }

    /// Inserts the `word` under the current node.
    ///
    /// If a part of the `word` is not yet present under the current node, that part is added. The
    /// already existing part of the `word` is unchanged.
    fn insert(&mut self, word: &str) {
        if let Some(root) = word.chars().next() {
            let prefix = self.value.clone();
            let root = self
                .children
                .entry(root)
                .or_insert_with(|| Node::new(root, format!("{}{}", prefix, root)));
            root.insert(&word[1..]);
        }
    }

    /// Deletes the word under the current node.
    ///
    /// Only the part that is not part of another word will be removed, if part of the word is a
    /// prefix of another word under the current node, that part will not be removed.
    fn delete(&mut self, word: &str) {
        if let Some(root) = word.chars().next() {
            if let Some(child) = self.children.get_mut(&root) {
                // If `child` doesn't have any children, it is the last node in the word, and can
                // thus safely be removed
                if child.children.len() == 0 {
                    self.children.remove(&root);
                } else {
                    // Firstly, try to delete the remainder of the word
                    child.delete(&word[1..]);

                    // Secondly, if `child` has no more children left, it can be safely removed.
                    // This can be the case when there are a few nodes with only 1 child, this
                    // takes care that we remove them recursively.
                    if child.children.len() == 0 {
                        self.children.remove(&root);
                    }
                }
            }
        }
    }

    /// Returns a reference to the [`Node`] containing the last character of the `word`.
    pub fn find(&self, word: &str) -> Option<&Self> {
        if let Some(root) = word.chars().next() {
            if let Some(child) = self.children.get(&root) {
                return child.find(&word[1..]);
            } else {
                return None;
            }
        }

        Some(self)
    }

    /// Returns a mutable reference to the [`Node`] containing the last character of the `word`.
    fn find_mut(&mut self, word: &str) -> Option<&mut Self> {
        if let Some(root) = word.chars().next() {
            if let Some(child) = self.children.get_mut(&root) {
                return child.find_mut(&word[1..]);
            } else {
                return None;
            }
        }

        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn insert_single() {
        let mut trie = Trie::new();
        let input = "Hello world!";

        trie.insert(input);
        assert!(trie.find(input).is_some());
        assert!(trie.find("Hi there").is_none());
    }

    #[test]
    fn insert_multiple() {
        let mut trie = Trie::new();
        let input1 = "Hello world!";
        let input2 = "Hello sir!";
        let input3 = "Good afternoon!";

        trie.insert(input1);
        trie.insert(input2);
        trie.insert(input3);

        assert!(trie.find(input1).is_some());
        assert!(trie.find(input2).is_some());
        assert!(trie.find(input3).is_some());
        assert!(trie.find("Hi there").is_none());
    }

    #[test]
    fn delete_single() {
        let mut trie = Trie::new();
        let input = "Hello world!";

        trie.insert(input);
        assert!(trie.find(input).is_some());

        trie.delete(input);
        assert!(trie.find(input).is_none());

        assert_eq!(len(&trie), 0);
    }

    #[test]
    fn delete_prefix() {
        let mut trie = Trie::new();
        let input = "Hello world!";

        trie.insert(input);
        assert!(trie.find(input).is_some());

        trie.delete("Hello");
        assert!(trie.find(input).is_some());

        assert_eq!(len(&trie), 1);
    }

    #[test]
    fn delete_after_prefix() {
        let mut trie = Trie::new();
        let input = "Hello world!";

        trie.insert(input);
        assert!(trie.find(input).is_some());

        trie.delete_after_prefix("Hello ", "world!");
        assert!(trie.find(input).is_none());

        assert_eq!(len(&trie), 1);
    }

    #[test]
    fn delete_multiple() {
        let mut trie = Trie::new();
        let input1 = "Hello world!";
        let input2 = "Hello sir!";
        let input3 = "Good afternoon!";

        trie.insert(input1);
        trie.insert(input2);
        trie.insert(input3);

        assert!(trie.find(input1).is_some());
        assert!(trie.find(input2).is_some());
        assert!(trie.find(input3).is_some());

        trie.delete(input1);
        assert!(trie.find(input1).is_none());

        trie.delete(input3);
        assert!(trie.find(input3).is_none());

        assert_eq!(len(&trie), 1);
    }

    #[test]
    fn find_in_empty_trie() {
        let trie = Trie::new();

        assert!(trie.find(" ").is_none());
    }

    #[test]
    fn find_prefix() {
        let mut trie = Trie::new();
        trie.insert("Hello world!");

        assert!(trie.find("Hello").is_some());
    }

    #[test]
    fn find_from_prefix() {
        let mut trie = Trie::new();
        trie.insert("Hello world!");
        trie.insert("Hello sir!");
        trie.insert("Hello miss!");

        if let Some(node) = trie.find("Hello ") {
            assert!(node.find("sir").is_some());
        }
    }

    fn len(trie: &Trie) -> usize {
        let mut len = 0;
        for _ in trie.words() {
            len += 1;
        }
        len
    }
}
