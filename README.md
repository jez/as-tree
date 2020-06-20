# as-tree

![CI](https://github.com/jRimbault/as-tree/workflows/CI/badge.svg)

Print a list of paths as a tree of paths.

For example, given:

```
dir1/foo.txt
dir1/bar.txt
dir2/qux.txt
```

it will print:

```
.
├── dir1
│   ├── foo.txt
│   └── bar.txt
└── dir2
    └── qux.txt
```

This tool is particularly useful when used with `find` or `fd` to produce such
a list of files. It's similar in spirit to `tree`, but `find` and `fd` tend to
be more powerful when it comes to controlling which files to list.

Inspired by [this feature request](https://github.com/sharkdp/fd/issues/283).

## Install

There are pre-built binary releases in the Releases tab.

To install from source using Cargo:

```shell
cargo install -f --git https://github.com/jez/as-tree
```

## Usage

```
❯ as-tree --help
Print a list of paths as a tree of paths.

Usage:
  as-tree [options] [<filename>]

Arguments:
  <filename>        The file to read from. When omitted, reads from stdin.

Options:
  --color (always|auto|never)
                    Whether to colorize the output [default: auto]
  -h, --help        Print this help message

Example:
  find . -name '*.txt' | as-tree
```

## Example

This tool is particularly useful with tools like `fd` which can prune the list
of files to print better than `tree` can alone.

```
❯ fd --exclude test | as-tree
.
├── LICENSE.md
├── Makefile
├── README.md
├── WORKSPACE
├── bazel
├── main
│   ├── BUILD
│   └── main.cc
├── third_party
│   ├── BUILD
│   ├── externals.bzl
│   └── spdlog.BUILD
└── tools
    ├── BUILD
    ├── clang.bzl
    └── scripts
        ├── build_compilation_db.sh
        └── generate_compdb_targets.sh
```

## Developing

```shell
cargo test
```

## TODO(jez)

- rustfmt / buildifier / shellcheck in CI
- [ ] Only use box drawing characters if the locale supports it
  - See `man locale`, `LC_CTYPE=C tree`
- [ ] Add a `-0` flag to support files with newlines in their name
  - Seriously why is this allowed?
