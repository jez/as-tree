mod trie;

pub use trie::PathTrie;

/// Specify how to display paths
#[derive(Copy, Clone, Debug)]
pub enum PathFormat {
    Normal,
    Long,
}

impl From<bool> for PathFormat {
    fn from(value: bool) -> Self {
        if value {
            PathFormat::Long
        } else {
            PathFormat::Normal
        }
    }
}

impl Default for PathFormat {
    fn default() -> Self {
        PathFormat::Normal
    }
}
