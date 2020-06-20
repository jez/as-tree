mod fixture;

use as_tree::PathFormat;
use as_tree::PathTrie;
use lscolors::LsColors;
use pretty_assertions::assert_eq;

#[test]
fn test_filesystem() {
    let files = "\
tests/dir1/bar.txt
tests/dir1/foo.txt
tests/dir2/qux.txt";
    let expected = "\
[1;34m[0m[1;34mtests[0m
├── [1;34mdir1[0m
│   ├── bar.txt
│   └── foo.txt
└── [1;34mdir2[0m
    └── qux.txt
";
    let trie: PathTrie = files.lines().collect();
    let result = format!(
        "{}",
        trie.custom_display(LsColors::default(), PathFormat::Normal)
    );
    assert_eq!(result, expected);
}

#[test]
fn test_full_paths() {
    let files = "\
LICENSE.md
README.md
WORKSPACE
bazel
main
main/BUILD
main/main.cc
test
test/BUILD
test/diff_one.sh
test/diff_tests.bzl
test/fixtures
test/fixtures/as-tree.txt
test/fixtures/symbol.txt
test/fixtures/symbol.txt.exp
test/update_one.sh
third_party
third_party/BUILD
third_party/externals.bzl
third_party/spdlog.BUILD
tools
tools/BUILD
tools/clang.bzl
tools/scripts
tools/scripts/build_compilation_db.sh
tools/scripts/generate_compdb_targets.sh";
    #[cfg(not(target_os = "windows"))]
    let expected = "\
.
├── ./LICENSE.md
├── ./README.md
├── ./WORKSPACE
├── ./bazel
├── ./main
│   ├── ./main/BUILD
│   └── ./main/main.cc
├── ./test
│   ├── ./test/BUILD
│   ├── ./test/diff_one.sh
│   ├── ./test/diff_tests.bzl
│   ├── ./test/fixtures
│   │   ├── ./test/fixtures/as-tree.txt
│   │   ├── ./test/fixtures/symbol.txt
│   │   └── ./test/fixtures/symbol.txt.exp
│   └── ./test/update_one.sh
├── ./third_party
│   ├── ./third_party/BUILD
│   ├── ./third_party/externals.bzl
│   └── ./third_party/spdlog.BUILD
└── ./tools
    ├── ./tools/BUILD
    ├── ./tools/clang.bzl
    └── ./tools/scripts
        ├── ./tools/scripts/build_compilation_db.sh
        └── ./tools/scripts/generate_compdb_targets.sh
";
    #[cfg(target_os = "windows")]
    let expected = "\
.
├── .\\LICENSE.md
├── .\\README.md
├── .\\WORKSPACE
├── .\\bazel
├── .\\main
│   ├── .\\main\\BUILD
│   └── .\\main\\main.cc
├── .\\test
│   ├── .\\test\\BUILD
│   ├── .\\test\\diff_one.sh
│   ├── .\\test\\diff_tests.bzl
│   ├── .\\test\\fixtures
│   │   ├── .\\test\\fixtures\\as-tree.txt
│   │   ├── .\\test\\fixtures\\symbol.txt
│   │   └── .\\test\\fixtures\\symbol.txt.exp
│   └── .\\test\\update_one.sh
├── .\\third_party
│   ├── .\\third_party\\BUILD
│   ├── .\\third_party\\externals.bzl
│   └── .\\third_party\\spdlog.BUILD
└── .\\tools
    ├── .\\tools\\BUILD
    ├── .\\tools\\clang.bzl
    └── .\\tools\\scripts
        ├── .\\tools\\scripts\\build_compilation_db.sh
        └── .\\tools\\scripts\\generate_compdb_targets.sh
";
    let trie: PathTrie = files.lines().collect();
    let result = format!(
        "{}",
        trie.custom_display(LsColors::empty(), PathFormat::Long)
    );
    assert_eq!(result, expected);
}
