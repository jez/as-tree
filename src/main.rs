mod color;

use as_tree::{PathFormat, PathTrie};
use color::Color;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

/// Print a list of paths as a tree of paths.
///
/// Example :
///   find . -name '*.txt' | as-tree
#[derive(Debug, Default, StructOpt)]
struct Options {
    /// The file to read from. When omitted, reads from stdin.
    filename: Option<PathBuf>,
    /// Whether to colorize the output, can only be [always|auto|never]
    #[structopt(short, long, default_value)]
    color: Color,
    /// Print the full path prefix for each file.
    #[structopt(short = "f", parse(from_flag))]
    path_format: PathFormat,
}

fn main() -> io::Result<()> {
    let options = Options::from_args();
    let trie = build_trie(options.filename)?;
    print!(
        "{}",
        trie.custom_display(options.color.into(), options.path_format)
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
