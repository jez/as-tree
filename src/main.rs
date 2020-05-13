extern crate ansi_term;
extern crate lscolors;

use std::collections::BTreeMap;
use std::env;
use std::fmt;
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

struct FmtContext<'a, 'b> {
    out: &'b mut fmt::Formatter<'b>,
    lscolors: &'a LsColors,
    path: PathBuf,
    prefix: &'a str,
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

    // TODO(jez) Put these three parameters into a helper struct
    fn _fmt(&self, ctx: &FmtContext) -> fmt::Result {
        let normal_prefix = format!("{}│   ", ctx.prefix);
        let last_prefix = format!("{}    ", ctx.prefix);

        for (idx, (current_path, it)) in self.trie.iter().enumerate() {
            let path = ctx.path.join(current_path);
            let style = ansi_style_for_path(ctx.lscolors, &path);
            let painted = style.paint(current_path.to_string_lossy());
            if idx != self.trie.len() - 1 {
                let ctx = FmtContext {
                    out: ctx.out,
                    lscolors: ctx.lscolors,
                    path: path,
                    prefix: &normal_prefix,
                };
                write!(ctx.out, "{}├── {}\n", ctx.prefix, painted)?;
                it._fmt(&ctx)?;
            } else {
                let ctx = FmtContext {
                    out: ctx.out,
                    lscolors: ctx.lscolors,
                    path: path,
                    prefix: &last_prefix,
                };
                write!(ctx.out, "{}└── {}\n", ctx.prefix, painted)?;
                it._fmt(&ctx)?;
            }
        }

        fmt::Result::Ok(())
    }
}

impl fmt::Display for PathTrie {
    fn fmt<'a>(&'a self, out: &mut fmt::Formatter) -> fmt::Result {
        if self.trie.is_empty() {
            return write!(out, "\n");
        }

        let lscolors = LsColors::from_env().unwrap_or_default();
        if let Some((path, it)) = self.trie.iter().next() {
            let ctx = FmtContext {
                out,
                lscolors: &lscolors,
                path: path.to_owned(),
                prefix: "",
            };
            let style = ansi_style_for_path(ctx.lscolors, path);

            if path.is_absolute() || path == &PathBuf::from(".") {
                write!(ctx.out, "{}\n", style.paint(path.to_string_lossy()))?;
                return it._fmt(&ctx);
            }
        }

        let ctx = FmtContext {
            out,
            lscolors: &lscolors,
            path: PathBuf::from("."),
            prefix: "",
        };
        let style = ansi_style_for_path(ctx.lscolors, Path::new("."));
        write!(ctx.out, "{}\n", style.paint("."))?;
        self._fmt(&ctx)
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
  <file>      The file to read from [default: stdin]
";

#[derive(Debug, Default)]
struct Options {
    pub filename: Option<String>,
}

fn parse_options_or_die() -> Options {
    let mut argv = env::args();

    if argv.next().is_none() {
        eprint!("{}", USAGE);
        exit(1);
    }

    let mut options = Options::default();
    for arg in argv {
        if arg.is_empty() {
            eprint!("Unrecognized argument: {}\n\n{}", arg, USAGE);
            exit(1);
        }

        if arg == "-h" || arg == "--help" {
            print!("{}", USAGE);
            exit(0);
        }

        if &arg[..1] == "-" {
            eprint!("Unrecognized option: {}\n\n{}", arg, USAGE);
            exit(1);
        }

        if options.filename.is_some() {
            eprint!("Extra argument: {}\n\n{}", arg, USAGE);
            exit(1);
        }

        options.filename = Some(arg.to_string());
    }

    return options;
}

fn main() -> io::Result<()> {
    let options = parse_options_or_die();

    let trie = match options.filename {
        None => drain_input_to_path_trie(&mut io::stdin().lock()),
        Some(filename) => {
            let file = File::open(filename)?;
            let mut reader = BufReader::new(file);
            drain_input_to_path_trie(&mut reader)
        }
    };

    print!("{}", trie);

    io::Result::Ok(())
}
