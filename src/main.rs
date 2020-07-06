mod color;

use argh::FromArgs;
use as_tree::{PathFormat, PathTrie};
use color::Color;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

/// Print a list of paths as a tree of paths.
///
/// Example :
///   find . -name '*.txt' | as-tree
#[derive(Debug, FromArgs)]
struct Options {
    /// the file to read from. When omitted, reads from stdin
    #[argh(positional)]
    filename: Option<PathBuf>,
    /// whether to colorize the output, can only be [always|auto|never]
    #[argh(short = 'c', option)]
    color: Option<Color>,
    /// print the full path prefix for each file [absolute|normal]
    #[argh(short = 'f', option)]
    path_format: Option<PathFormat>,
}

fn main() -> io::Result<()> {
    let options: Options = argh::from_env();
    let trie = build_trie(options.filename)?;
    println!(
        "{}",
        trie.custom_display(
            options.color.unwrap_or_default().into(),
            options.path_format.unwrap_or_default()
        )
    );
    Ok(())
}

fn build_trie(filename: Option<PathBuf>) -> io::Result<PathTrie> {
    let trie = match filename {
        None => {
            if atty::is(atty::Stream::Stdin) {
                eprintln!("Warning: reading from stdin, which is a tty.");
            }
            read_lines_from_buffer(io::stdin().lock())
        }
        Some(filename) => {
            let reader = BufReader::new(File::open(filename)?);
            read_lines_from_buffer(reader)
        }
    };
    Ok(trie)
}

fn read_lines_from_buffer<T: BufRead>(input: T) -> PathTrie {
    input.lines().filter_map(Result::ok).collect()
}
