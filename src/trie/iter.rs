use super::PathTrie;
use std::iter::FromIterator;
use std::path::Path;

impl<P: AsRef<Path>> FromIterator<P> for PathTrie {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        let mut trie = Self::default();
        for path in iter {
            trie.insert(path);
        }
        trie
    }
}
