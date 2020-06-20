#[test]
fn test() {
    let paths = "\
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
tools/scripts/generate_compdb_targets.sh
";
    let expected = "\
.
├── LICENSE.md
├── README.md
├── WORKSPACE
├── bazel
├── main
│   ├── BUILD
│   └── main.cc
├── test
│   ├── BUILD
│   ├── diff_one.sh
│   ├── diff_tests.bzl
│   ├── fixtures
│   │   ├── as-tree.txt
│   │   ├── symbol.txt
│   │   └── symbol.txt.exp
│   └── update_one.sh
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
";
    super::common_test(paths, expected);
}
