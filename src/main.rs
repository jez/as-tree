extern crate ansi_term;
extern crate atty;
extern crate lscolors;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use lscolors::{LsColors, Style};

pub mod options;

#[derive(Debug, Default)]
pub struct PathTrie {
    // We rely on the sorted iteration order
    trie: BTreeMap<PathBuf, PathTrie>,
}

fn ansi_style_for_path(lscolors: &LsColors, path: &Path) -> ansi_term::Style {
    lscolors
        .style_for_path(&path)
        .map(Style::to_ansi_term_style)
        .unwrap_or_default()
}

impl PathTrie {
    fn contains_singleton_dir(&self) -> bool {
        self.trie.len() == 1 && !self.trie.iter().next().unwrap().1.trie.is_empty()
    }

    pub fn insert(&mut self, path: &Path) {
        let mut cur = self;
        for comp in path.iter() {
            cur = cur
                .trie
                .entry(PathBuf::from(comp))
                .or_insert_with(PathTrie::default);
        }
    }

    fn _print(
        &self,
        top: bool,
        prefix: &str,
        join_with_parent: bool,
        lscolors: &LsColors,
        parent_path: PathBuf,
        full_path: bool
    ) {
        let normal_prefix = format!("{}│   ", prefix);
        let last_prefix = format!("{}    ", prefix);

        for (idx, (path, it)) in self.trie.iter().enumerate() {
            let current_path = parent_path.join(path);
            let style = ansi_style_for_path(&lscolors, &current_path);
            let painted = match full_path {
                false => style.paint(path.to_string_lossy()),
                true => style.paint(current_path.to_string_lossy()),
            };

            let contains_singleton_dir = it.contains_singleton_dir();
            let newline = if contains_singleton_dir { "" } else { "\n" };
            let is_last = idx == self.trie.len() - 1;

            let next_prefix = if join_with_parent {
                let joiner = if top || parent_path == PathBuf::from("/") {
                    ""
                } else {
                    "/"
                };
                print!("{}{}{}", style.paint(joiner), painted, newline);
                prefix
            } else if !is_last {
                print!("{}├── {}{}", prefix, painted, newline);
                &normal_prefix
            } else {
                print!("{}└── {}{}", prefix, painted, newline);
                &last_prefix
            };

            it._print(
                false,
                next_prefix,
                contains_singleton_dir,
                lscolors,
                current_path,
                full_path
            )
        }
    }

    fn print(&self, lscolors: &LsColors, full_path: bool) {
        if self.trie.is_empty() {
            println!();
            return;
        }

        // This works because PathBuf::from(".").join(PathBuf::from("/")) == PathBuf::from("/")
        let current_path = PathBuf::from(".");
        let contains_singleton_dir = self.contains_singleton_dir();

        if !contains_singleton_dir {
            let style = ansi_style_for_path(&lscolors, &current_path);
            println!("{}", style.paint(current_path.to_string_lossy()));
        }

        self._print(true, "", contains_singleton_dir, &lscolors, current_path, full_path)
    }
}

fn drain_input_to_path_trie<T: BufRead>(input: &mut T) -> PathTrie {
    let mut trie = PathTrie::default();

    for path_buf in input.lines().filter_map(Result::ok).map(PathBuf::from) {
        trie.insert(&path_buf)
    }

    trie
}

fn main() -> io::Result<()> {
    let options = options::parse_options_or_die();

    let trie = match &options.filename {
        None => {
            if atty::is(atty::Stream::Stdin) {
                eprintln!("Warning: reading from stdin, which is a tty.");
            }
            drain_input_to_path_trie(&mut io::stdin().lock())
        }
        Some(filename) => {
            let file = File::open(filename)?;
            let mut reader = BufReader::new(file);
            drain_input_to_path_trie(&mut reader)
        }
    };

    let lscolors = match &options.colorize {
        options::Colorize::Always => LsColors::from_env().unwrap_or_default(),
        options::Colorize::Auto => {
            if atty::is(atty::Stream::Stdout) {
                LsColors::from_env().unwrap_or_default()
            } else {
                LsColors::empty()
            }
        }
        options::Colorize::Never => LsColors::empty(),
    };

    trie.print(&lscolors, options.full_path);

    io::Result::Ok(())
}
