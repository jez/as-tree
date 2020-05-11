# as-tree

Print a list of paths as a tree of paths.

For example, given:

```
dir1/foo.txt
dir1/bar.txt
dir3/qux.txt
```

it will print:

```
.
├── dir1
│   ├── foo.txt
│   └── bar.txt
└── dir2
```

This tools is particularly useful when used with `find` or `fd` to produce such
a list of files. It's similar in spirit to `tree`, but `find` and `fd` tend to
be more powerful when it comes to controling which files to list.

Inspired by [this feature request](https://github.com/sharkdp/fd/issues/283).

## Install

This project is written in Rust and built using Bazel. The Makefile will
download all the tools you need to build it, including Bazel and Rust.

```shell
# Build from source, installs to ~/.local/bin/as-tree
make install

# Build from source, installs to /usr/local/bin/as-tree
make install prefix=/usr/local
```

## Usage

```
❯ as-tree --help
Print a list of paths as a tree of paths.

Usage:
  as-tree [<file>]

Arguments:
  <file>      The file to read from [default: stdin]
```

## Example

This tools is particularly useful with tools like `fd` which can prune the list
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
# Fast build (some debug info, but fast compile times):
./bazel build //src:as-tree

# Run the tests:
./bazel test --test_output=errors //test

# To add a test, create two files:
#
# - test/fixtures/foo.txt
# - test/fixtures/foo.txt.exp
#
# The first file is the input to feed to `as-tree`, and the second is the
# expected output of `as-tree` on that input

# Update all the tests:
./bazel test //test:update
```

## TODO(jez)

- Set up CI (should build under both Bazel and Cargo)
- rustfmt / buildifier / shellcheck in CI
- tests for CLI options
- [ ] Color output according to LS_COLORS environment variable. Prior art:
  - <https://github.com/sharkdp/lscolors#references>
  - <https://github.com/tavianator/bfs/blob/2d3b03183c9f1/color.c#L308>
- [ ] Only use box drawing characters if the locale supports it
  - See `man locale`, `LC_CTYPE=C tree`
- [ ] Collapse consecutive singleton tries into one level
  - `tree` also does this
