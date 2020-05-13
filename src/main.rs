extern crate ansi_term;
extern crate atty;
extern crate lscolors;
mod options;

use lscolors::{LsColors, Style};
use options::Options;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

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
    pub fn insert(&mut self, path: &Path) {
        let mut cur = self;
        for comp in path.iter() {
            cur = cur
                .trie
                .entry(PathBuf::from(comp))
                .or_insert_with(PathTrie::default);
        }
    }

    fn _print(&self, prefix: &str, lscolors: &LsColors, parent_path: PathBuf) {
        let normal_prefix = format!("{}│   ", prefix);
        let last_prefix = format!("{}    ", prefix);

        for (idx, (path, it)) in self.trie.iter().enumerate() {
            let current_path = parent_path.join(path);
            let style = ansi_style_for_path(&lscolors, &current_path);
            let painted = style.paint(path.to_string_lossy());
            if idx != self.trie.len() - 1 {
                println!("{}├── {}", prefix, painted);
                it._print(&normal_prefix, lscolors, current_path);
            } else {
                println!("{}└── {}", prefix, painted);
                it._print(&last_prefix, lscolors, current_path);
            }
        }
    }

    fn print(&self, lscolors: &LsColors) {
        if self.trie.is_empty() {
            println!("");
            return;
        }

        if let Some((path, it)) = self.trie.iter().next() {
            if path.is_absolute() || path == &PathBuf::from(".") {
                let style = ansi_style_for_path(&lscolors, &path);
                let painted = style.paint(path.to_string_lossy());
                println!("{}", painted);
                it._print("", &lscolors, path.to_owned());
                return;
            }
        }

        let style = ansi_style_for_path(&lscolors, Path::new("."));
        println!("{}", style.paint("."));
        self._print("", &lscolors, PathBuf::from("."));
    }
}

fn drain_input_to_path_trie<T: BufRead>(input: &mut T) -> PathTrie {
    let mut trie = PathTrie::default();

    for path_buf in input.lines().filter_map(Result::ok).map(PathBuf::from) {
        trie.insert(&path_buf)
    }

    return trie;
}

fn main() -> io::Result<()> {
    let options = Options::from_args();

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

    trie.print(&lscolors);

    io::Result::Ok(())
}
