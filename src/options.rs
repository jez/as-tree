use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Clone, Debug, PartialEq)]
pub enum Colorize {
    Always,
    Auto,
    Never,
}

impl Default for Colorize {
    fn default() -> Self {
        Colorize::Auto
    }
}

impl FromStr for Colorize {
    type Err = String;

    fn from_str(value: &str) -> Result<Colorize, Self::Err> {
        match value {
            "always" => Ok(Colorize::Always),
            "auto" => Ok(Colorize::Auto),
            "never" => Ok(Colorize::Never),
            _ => Err(format!(
                "color option can only be [always|auto|never], found : {}",
                value
            )),
        }
    }
}

impl fmt::Display for Colorize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colorize::Auto => "auto".fmt(f),
            Colorize::Always => "always".fmt(f),
            Colorize::Never => "never".fmt(f),
        }
    }
}

/// Print a list of paths as a tree of paths.
///
/// Example :
///   find . -name '*.txt' | as-tree
#[derive(Clone, Debug, Default, StructOpt)]
pub struct Options {
    /// The file to read from. When omitted, reads from stdin.
    pub filename: Option<PathBuf>,
    /// Whether to colorize the output
    #[structopt(short, long = "color", default_value)]
    pub colorize: Colorize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parse() {
        let opts = Options::from_iter_safe("as-tree file".split_whitespace());
        assert!(opts.is_ok());
        let opts = Options::from_iter_safe("as-tree file -c never".split_whitespace());
        assert!(opts.is_ok());
        let opts = Options::from_iter_safe("as-tree file --color auto".split_whitespace());
        assert!(opts.is_ok());
        let opts = Options::from_iter_safe("as-tree --color never file".split_whitespace());
        assert!(opts.is_ok());
        let opts = Options::from_iter_safe("as-tree".split_whitespace());
        assert!(opts.is_ok());
        let opts = Options::from_iter_safe("as-tree -c never".split_whitespace());
        assert!(opts.is_ok());
        let opts = Options::from_iter_safe("as-tree --color auto".split_whitespace());
        assert!(opts.is_ok());
        let opts = Options::from_iter_safe("as-tree --color never".split_whitespace());
        assert!(opts.is_ok());
    }

    #[test]
    fn incorrect_parse() {
        let opts = Options::from_iter_safe("as-tree file file".split_whitespace());
        assert!(opts.is_err());
    }
}
