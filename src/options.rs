use std::env;
use std::process::exit;
use std::str::FromStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum Colorize {
    Always,
    Auto,
    Never,
}

impl FromStr for Colorize {
    type Err = ();

    fn from_str(color: &str) -> Result<Self, Self::Err> {
        match color {
            "always" => Ok(Colorize::Always),
            "auto" => Ok(Colorize::Auto),
            "never" => Ok(Colorize::Never),
            _ => Err(()),
        }
    }
}

impl Default for Colorize {
    fn default() -> Self {
        Colorize::Auto
    }
}

#[derive(Debug, Default)]
pub struct Options {
    pub files: Vec<String>,
    pub colorize: Colorize,
}

const USAGE: &str = "\
Print a list of paths as a tree of paths.

Usage:
  as-tree [options] [<files>]

Arguments:
  <files...>        The files to read from. When omitted, reads from stdin.

Options:
  --color (always|auto|never)
                    Whether to colorize the output [default: auto]
  -h, --help        Print this help message
  -v, --version     Print the version and exit

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

        if arg == "-v" || arg == "--version" {
            println!("{}", VERSION);
            exit(0);
        }

        if arg == "--color" {
            if let Some(color) = argv.next() {
                match color.parse() {
                    Ok(colorize) => options.colorize = colorize,
                    Err(()) => die("Unrecognized option: --color", &color),
                }
            } else {
                die("-> Unrecognized option:", &arg);
            }
            continue;
        }

        if &arg[..1] == "-" {
            die("Unrecognized option:", &arg);
        }

        options.files.push(arg);
    }

    options
}
