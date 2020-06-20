#[test]
fn test() {
    let paths = "\
/Users/jez/prog/cli/as-tree/LICENSE.md
/Users/jez/prog/cli/as-tree/Makefile
/Users/jez/prog/cli/as-tree/README.md
/Users/jez/prog/cli/as-tree/WORKSPACE
/Users/jez/prog/cli/as-tree/bazel
/Users/jez/prog/cli/as-tree/main
/Users/jez/prog/cli/as-tree/main/BUILD
/Users/jez/prog/cli/as-tree/main/main.cc
/Users/jez/prog/cli/as-tree/test
/Users/jez/prog/cli/as-tree/test/BUILD
/Users/jez/prog/cli/as-tree/test/diff_one.sh
/Users/jez/prog/cli/as-tree/test/diff_tests.bzl
/Users/jez/prog/cli/as-tree/test/fixtures
/Users/jez/prog/cli/as-tree/test/fixtures/as-tree-absolute.txt
/Users/jez/prog/cli/as-tree/test/fixtures/as-tree.txt
/Users/jez/prog/cli/as-tree/test/fixtures/as-tree.txt.exp
/Users/jez/prog/cli/as-tree/test/fixtures/empty.txt
/Users/jez/prog/cli/as-tree/test/fixtures/empty.txt.exp
/Users/jez/prog/cli/as-tree/test/fixtures/sorbet-extension-c-h-cc-hh.txt
/Users/jez/prog/cli/as-tree/test/fixtures/sorbet-extension-c-h-cc-hh.txt.exp
/Users/jez/prog/cli/as-tree/test/fixtures/sorbet-extension-md.txt
/Users/jez/prog/cli/as-tree/test/fixtures/sorbet-extension-md.txt.exp
/Users/jez/prog/cli/as-tree/test/fixtures/sorbet.txt
/Users/jez/prog/cli/as-tree/test/fixtures/sorbet.txt.exp
/Users/jez/prog/cli/as-tree/test/fixtures/symbol.txt
/Users/jez/prog/cli/as-tree/test/fixtures/symbol.txt.exp
/Users/jez/prog/cli/as-tree/test/update_one.sh
/Users/jez/prog/cli/as-tree/third_party
/Users/jez/prog/cli/as-tree/third_party/BUILD
/Users/jez/prog/cli/as-tree/third_party/externals.bzl
/Users/jez/prog/cli/as-tree/tools
/Users/jez/prog/cli/as-tree/tools/BUILD
/Users/jez/prog/cli/as-tree/tools/clang.bzl
/Users/jez/prog/cli/as-tree/tools/scripts
/Users/jez/prog/cli/as-tree/tools/scripts/build_compilation_db.sh
/Users/jez/prog/cli/as-tree/tools/scripts/generate_compdb_targets.sh";
    #[cfg(target_os = "windows")]
    let expected = "\
\\Users\\jez\\prog\\cli\\as-tree
├── LICENSE.md
├── Makefile
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
│   │   ├── as-tree-absolute.txt
│   │   ├── as-tree.txt
│   │   ├── as-tree.txt.exp
│   │   ├── empty.txt
│   │   ├── empty.txt.exp
│   │   ├── sorbet-extension-c-h-cc-hh.txt
│   │   ├── sorbet-extension-c-h-cc-hh.txt.exp
│   │   ├── sorbet-extension-md.txt
│   │   ├── sorbet-extension-md.txt.exp
│   │   ├── sorbet.txt
│   │   ├── sorbet.txt.exp
│   │   ├── symbol.txt
│   │   └── symbol.txt.exp
│   └── update_one.sh
├── third_party
│   ├── BUILD
│   └── externals.bzl
└── tools
    ├── BUILD
    ├── clang.bzl
    └── scripts
        ├── build_compilation_db.sh
        └── generate_compdb_targets.sh
";
    #[cfg(not(target_os = "windows"))]
    let expected = "\
/Users/jez/prog/cli/as-tree
├── LICENSE.md
├── Makefile
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
│   │   ├── as-tree-absolute.txt
│   │   ├── as-tree.txt
│   │   ├── as-tree.txt.exp
│   │   ├── empty.txt
│   │   ├── empty.txt.exp
│   │   ├── sorbet-extension-c-h-cc-hh.txt
│   │   ├── sorbet-extension-c-h-cc-hh.txt.exp
│   │   ├── sorbet-extension-md.txt
│   │   ├── sorbet-extension-md.txt.exp
│   │   ├── sorbet.txt
│   │   ├── sorbet.txt.exp
│   │   ├── symbol.txt
│   │   └── symbol.txt.exp
│   └── update_one.sh
├── third_party
│   ├── BUILD
│   └── externals.bzl
└── tools
    ├── BUILD
    ├── clang.bzl
    └── scripts
        ├── build_compilation_db.sh
        └── generate_compdb_targets.sh
";
    super::common_test(paths, expected);
}
