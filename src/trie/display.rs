use super::super::PathFormat;
use super::PathTrie;
use lscolors::{LsColors, Style};
use std::fmt;
use std::path::Path;

/// Helper struct for safely displaying a [PathTrie](structs.PathTrie)
#[derive(Debug)]
pub struct DisplayTrie<'a> {
    path_trie: &'a PathTrie,
    options: DisplayOptions,
}

#[derive(Debug)]
struct DisplayOptions {
    path_format: PathFormat,
    colors: LsColors,
    separator: String,
}

impl<'a> DisplayTrie<'a> {
    pub fn new(path_trie: &'a PathTrie, colors: LsColors, path_format: PathFormat) -> Self {
        Self {
            path_trie,
            options: DisplayOptions {
                path_format,
                colors,
                separator: std::path::MAIN_SEPARATOR.to_string(),
            },
        }
    }
}

impl<'a> fmt::Display for DisplayTrie<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.path_trie.trie.is_empty() {
            writeln!(f)
        } else {
            let current_path: &Path = ".".as_ref();
            let contains_singleton_dir = self.path_trie.contains_singleton_dir();

            if !contains_singleton_dir {
                let style = ansi_style_for_path(&self.options.colors, &current_path);
                writeln!(f, "{}", style.paint(current_path.to_string_lossy()))?;
            }

            fmt::Display::fmt(
                &SubTrie {
                    parent_trie: &self.path_trie,
                    top: true,
                    prefix: "",
                    join_with_parent: contains_singleton_dir,
                    options: &self.options,
                    parent_path: current_path,
                },
                f,
            )
        }
    }
}

#[derive(Debug)]
struct SubTrie<'a> {
    options: &'a DisplayOptions,
    parent_trie: &'a PathTrie,
    top: bool,
    prefix: &'a str,
    join_with_parent: bool,
    parent_path: &'a Path,
}

impl<'a> fmt::Display for SubTrie<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let normal_prefix = format!("{}│   ", self.prefix);
        let last_prefix = format!("{}    ", self.prefix);

        for (idx, (path, sub_trie)) in self.parent_trie.trie.iter().enumerate() {
            let current_path = self.parent_path.join(path);
            let style = ansi_style_for_path(&self.options.colors, &current_path);
            let painted = match self.options.path_format {
                PathFormat::Normal => path.to_string_lossy(),
                PathFormat::Long => current_path.to_string_lossy(),
            };
            let painted = style.paint(painted);

            let contains_singleton_dir = sub_trie.contains_singleton_dir();
            let newline = if contains_singleton_dir { "" } else { "\n" };
            let is_last = idx == self.parent_trie.trie.len() - 1;

            let next_prefix = if self.join_with_parent {
                let slash: &Path = self.options.separator.as_ref();
                let joiner = if self.top || self.parent_path == slash {
                    ""
                } else {
                    &self.options.separator
                };
                write!(f, "{}{}{}", style.paint(joiner), painted, newline)?;
                self.prefix
            } else if !is_last {
                write!(f, "{}├── {}{}", self.prefix, painted, newline)?;
                &normal_prefix
            } else {
                write!(f, "{}└── {}{}", self.prefix, painted, newline)?;
                &last_prefix
            };

            fmt::Display::fmt(
                &SubTrie {
                    parent_trie: sub_trie,
                    top: false,
                    prefix: next_prefix,
                    join_with_parent: contains_singleton_dir,
                    options: self.options,
                    parent_path: &current_path,
                },
                f,
            )?;
        }

        Ok(())
    }
}

fn ansi_style_for_path<P: AsRef<Path>>(colors: &LsColors, path: P) -> ansi_term::Style {
    colors
        .style_for_path(path) // syscall here
        .map(Style::to_ansi_term_style)
        .unwrap_or_default()
}
