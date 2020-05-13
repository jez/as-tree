extern crate ansi_term;
extern crate lscolors;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;

use lscolors::{LsColors, Style};

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

const USAGE: &'static str = "\
Print a list of paths as a tree of paths.

Usage:
  as-tree [<file>]

Arguments:
  <file>                  The file to read from [default: stdin]

Options:
  --color=(always|never)  Whether to colorize the output [default: false]
";

#[derive(Debug, Default)]
struct Options {
    pub filename: Option<String>,
    // TODO(jez) Infer whether to use color from isatty
    pub color: bool,
}

fn parse_options_or_die() -> Options {
    fn die(msg: &str, arg: &str) -> ! {
        eprint!("{} '{}'\n\n{}", msg, arg, USAGE);
        exit(1);
    }

    let mut argv = env::args();

    if argv.next().is_none() {
        eprint!("{}", USAGE);
        exit(1);
    }

    let mut options = Options::default();
    while let Some(arg) = argv.next() {
        if arg.is_empty() {
            die("Unrecognized argument:", &arg);
        }

        if arg == "-h" || arg == "--help" {
            print!("{}", USAGE);
            exit(0);
        }

        if arg == "--color=always" {
            options.color = true;
            continue;
        } else if arg == "--color=never" {
            options.color = false;
            continue;
        }

        if arg == "--color" {
            if let Some(color) = argv.next() {
                if color == "always" {
                    options.color = true;
                    continue;
                } else if color == "never" {
                    options.color = false;
                    continue;
                } else {
                    die("Unrecognized option: --color", &color);
                }
            } else {
                die("-> Unrecognized option:", &arg);
            }
        }

        if &arg[..1] == "-" {
            die("Unrecognized option:", &arg);
        }

        if options.filename.is_some() {
            die("Extra argument:", &arg);
        }

        options.filename = Some(arg.to_string());
    }

    return options;
}

fn main() -> io::Result<()> {
    let options = parse_options_or_die();

    let trie = match &options.filename {
        None => drain_input_to_path_trie(&mut io::stdin().lock()),
        Some(filename) => {
            let file = File::open(filename)?;
            let mut reader = BufReader::new(file);
            drain_input_to_path_trie(&mut reader)
        }
    };

    let lscolors = if options.color {
        LsColors::from_env().unwrap_or_default()
    } else {
        LsColors::empty()
    };

    trie.print(&lscolors);

    io::Result::Ok(())
}
