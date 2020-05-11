use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug, Default)]
pub struct PathTrie {
    // We rely on the sorted iteration order
    trie: BTreeMap<PathBuf, PathTrie>,
}

impl PathTrie {
    pub fn insert(&mut self, path: &Path) {
        let mut cur = self;
        for comp in path.iter() {
            cur = cur
                .trie
                .entry(PathBuf::from(comp))
                .or_insert_with(PathTrie::default);
        }
    }

    fn _fmt(&self, out: &mut fmt::Formatter, outer_prefix: &str) -> fmt::Result {
        // TODO(jez) Handle non-UTF-8 locales
        let normal_prefix = format!("{}│   ", outer_prefix);
        let last_prefix = format!("{}    ", outer_prefix);

        for (idx, (path, it)) in self.trie.iter().enumerate() {
            if idx != self.trie.len() - 1 {
                write!(out, "{}├── {}\n", outer_prefix, path.display())?;
                it._fmt(out, &normal_prefix)?;
            } else {
                write!(out, "{}└── {}\n", outer_prefix, path.display())?;
                it._fmt(out, &last_prefix)?;
            }
        }

        fmt::Result::Ok(())
    }
}

impl fmt::Display for PathTrie {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self._fmt(out, "")
    }
}

const usage: &'static str = "\
Print a list of paths as a tree of paths.

Usage:
  as-tree [<file>]

Arguments:
  <file>      The file to read from [default: stdin]
";

#[derive(Debug, Default)]
struct Options {
    pub filename: Option<String>,
}

fn parse_options_or_die() {
    let argv = env::args();

    if argv.next().is_none() {
        eprint!("{}", usage);
        exit(1);
    }

    let mut options = Options::default;
    for arg in argv {
        if arg.is_empty() {
            eprint!("Unrecognized argument: {}\n\n{}", arg, usage);
            exit(1);
        }

        if arg == "-h" || arg == "--help" {
            print!("{}", usage);
            exit(0);
        }

        if arg[..1] = "-" {
            eprint!("Unrecognized option: {}\n\n{}", arg, usage);
            exit(1);
        }

        if options.filename.is_some() {
            eprint!("Extra argument: {}\n\n{}", arg, usage);
            exit(1);
        }

        options.filename = Some(arg.to_string());
    }

    return options;
}

fn main() {
    let mut trie = PathTrie::default();
    for path_buf in io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(PathBuf::from)
    {
        trie.insert(&path_buf)
    }

    print!(".\n{}", trie);
}
