use std::env;
use std::process::exit;

#[derive(Debug)]
pub enum Colorize {
    Never,
    Always,
}

impl Default for Colorize {
    fn default() -> Self {
        Colorize::Never
    }
}

#[derive(Debug, Default)]
pub struct Options {
    pub filename: Option<String>,
    pub colorize: Colorize,
}

const USAGE: &'static str = "\
Print a list of paths as a tree of paths.

Usage:
  as-tree [options] [<filename>]

Arguments:
  <filename>              The file to read from. When omitted, reads from stdin.

Options:
  --color (always|never)  Whether to colorize the output [default: never]
  -h, --help              Print this help message

Example:
  find . -name '*.txt' | as-tree
";

pub fn parse_options_or_die() -> Options {
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

        if arg == "--color" {
            if let Some(color) = argv.next() {
                if color == "always" {
                    options.colorize = Colorize::Always;
                } else if color == "never" {
                    options.colorize = Colorize::Never;
                } else {
                    die("Unrecognized option: --color", &color);
                }
            } else {
                die("-> Unrecognized option:", &arg);
            }
            continue;
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
