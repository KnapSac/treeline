//! This module provides the [`Trie`] datastructure, a type of search tree.
#![warn(missing_docs, broken_intra_doc_links)]

use std::collections::HashMap;

/// The `Trie` datastructure.
///
/// The current implementation uses [`Node`]s to store the values inside the trie. Each [`Node`]
/// has a key associated with it, by traversing the trie depth-first, words can be found inside the
/// trie. Each [`Node`] keeps track of its children using a [`HashMap`], using the key of a child
/// as the key for the [`HashMap`].
#[derive(Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    /// Create an empty trie datastructure.
    pub fn new() -> Self {
        Self {
            root: Node::new(' ', String::new()),
        }
    }

    /// Inserts the input into the trie.
    ///
    /// If a part of the input is not yet present in the trie, that part is added. The already
    /// existing part of the input is unchanged.
    pub fn insert(&mut self, input: &str) {
        self.root.insert(input);
    }

    /// Returns a reference to the [`Node`] containing the last character of the input.
    pub fn find(&self, input: &str) -> Option<&Node> {
        self.root.find(input)
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
/// This iterator is returned from the [`Trie::words_with_prefix`] function on a [`Trie`] and will yield
/// instances of [`String`].
pub struct TrieRead<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for TrieRead<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(head) = self.stack.pop() {
                for child in head.children.values() {
                    self.stack.push(child);
                }

                if head.children.is_empty() {
                    return Some(&head.value);
                }
            } else {
                break;
            }
        }

        None
    }
}

/// A `Node` in a [`Trie`].
#[derive(Debug)]
pub struct Node {
    key: char,
    value: String,
    children: HashMap<char, Node>,
}

impl Node {
    /// Creates a new `Node` with the given key.
    pub fn new(key: char, value: String) -> Self {
        Self {
            key,
            value,
            children: HashMap::new(),
        }
    }

    /// Inserts the input under the current node.
    ///
    /// If a part of the input is not yet present under the current node, that part is added. The
    /// already existing part of the input is unchanged.
    pub fn insert(&mut self, input: &str) {
        if let Some(root) = input.chars().next() {
            let prefix = self.value.clone();
            let root = self
                .children
                .entry(root)
                .or_insert_with(|| Node::new(root, format!("{}{}", prefix, root)));
            root.insert(&input[1..]);
        }
    }

    /// Returns a reference to the [`Node`] containing the last character of the input.
    pub fn find(&self, input: &str) -> Option<&Self> {
        if let Some(root) = input.chars().next() {
            if let Some(child) = self.children.get(&root) {
                return child.find(&input[1..]);
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
}
