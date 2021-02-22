extern crate ansi_term;
extern crate atty;
extern crate lscolors;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
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
        self.trie.len() == 1 && !self.trie.values().next().unwrap().trie.is_empty()
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
        output_writer: &mut impl Write,
    ) -> io::Result<()> {
        let normal_prefix = format!("{}│   ", prefix);
        let last_prefix = format!("{}    ", prefix);

        for (idx, (path, it)) in self.trie.iter().enumerate() {
            let current_path = parent_path.join(path);
            let style = ansi_style_for_path(&lscolors, &current_path);
            let painted = style.paint(path.to_string_lossy());

            let contains_singleton_dir = it.contains_singleton_dir();
            let newline = if contains_singleton_dir { "" } else { "\n" };
            let is_last = idx == self.trie.len() - 1;

            let next_prefix = if join_with_parent {
                let joiner = if top || parent_path == PathBuf::from("/") {
                    ""
                } else {
                    "/"
                };
                write!(
                    output_writer,
                    "{}{}{}",
                    style.paint(joiner),
                    painted,
                    newline
                )?;
                prefix
            } else if !is_last {
                write!(output_writer, "{}├── {}{}", prefix, painted, newline)?;
                &normal_prefix
            } else {
                write!(output_writer, "{}└── {}{}", prefix, painted, newline)?;
                &last_prefix
            };

            it._print(
                false,
                next_prefix,
                contains_singleton_dir,
                lscolors,
                current_path,
                output_writer,
            )?;
        }
        Ok(())
    }

    fn print(&self, lscolors: &LsColors, mut output_writer: impl Write) {
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

        self._print(
            true,
            "",
            contains_singleton_dir,
            &lscolors,
            current_path,
            &mut output_writer,
        )
        .expect("Error while trying to output text");
    }
}

fn drain_inputs_to_path_trie<T: BufRead>(inputs: &mut [T]) -> PathTrie {
    let mut trie = PathTrie::default();

    let lines = inputs.iter_mut().flat_map(|input| input.lines());

    for line in lines.filter_map(Result::ok) {
        trie.insert(&PathBuf::from(line))
    }
    trie
}

fn main() -> io::Result<()> {
    let options = options::parse_options_or_die();

    let mut files: Vec<Box<dyn BufRead>> = if options.files.is_empty() {
        if atty::is(atty::Stream::Stdin) {
            eprintln!("Warning: reading from stdin, which is a tty.");
        }
        vec![Box::new(BufReader::new(io::stdin()))]
    } else {
        let mut readers: Vec<Box<dyn BufRead>> = vec![];
        for file in options.files {
            let file = File::open(file)?;
            readers.push(Box::new(BufReader::new(file)));
        }
        readers
    };

    let trie = drain_inputs_to_path_trie(&mut files);

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

    let output_writer = BufWriter::new(io::stdout());
    trie.print(&lscolors, output_writer);

    io::Result::Ok(())
}
