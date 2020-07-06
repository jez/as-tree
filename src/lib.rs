mod trie;

pub use trie::PathTrie;

/// Specify how to display paths
#[derive(Copy, Clone, Debug)]
pub enum PathFormat {
    Normal,
    Absolute,
}

impl From<bool> for PathFormat {
    fn from(value: bool) -> Self {
        if value {
            PathFormat::Absolute
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

impl std::str::FromStr for PathFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "absolute" => Ok(PathFormat::Absolute),
            "normal" => Ok(PathFormat::Normal),
            _ => Ok(PathFormat::Normal),
        }
    }
}
