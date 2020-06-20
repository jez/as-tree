mod display;
mod iter;

use super::PathFormat;
use display::DisplayTrie;
use lscolors::LsColors;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Collection of [paths](std::path).
///
/// # Examples :
///
/// ```
/// use as_tree::PathTrie;
///
/// let trie: PathTrie = "".lines().collect();
/// let trie: PathTrie = [""].iter().collect();
/// ```
#[derive(Debug, Default)]
pub struct PathTrie {
    /// We rely on the sorted iteration order
    trie: BTreeMap<PathBuf, PathTrie>,
}

impl PathTrie {
    /// Adds a path to the internal collection.
    pub fn insert<P: AsRef<Path>>(&mut self, path: P) {
        let mut current = self;
        for comp in path.as_ref().iter() {
            current = current
                .trie
                .entry(comp.into())
                .or_insert_with(Self::default);
        }
    }

    fn contains_singleton_dir(&self) -> bool {
        self.trie.len() == 1 && !self.trie.iter().next().unwrap().1.trie.is_empty()
    }

    /// Returns a struct that implements [`Display`](std::fmt::Display) for printing.
    pub fn display(&self) -> DisplayTrie<'_> {
        DisplayTrie::new(self, LsColors::empty(), PathFormat::Normal)
    }

    /// Returns a struct that implements [`Display`](std::fmt::Display) for printing,
    /// with colors and custom formatting.
    pub fn custom_display(&self, colors: LsColors, path_format: PathFormat) -> DisplayTrie<'_> {
        DisplayTrie::new(self, colors, path_format)
    }
}
