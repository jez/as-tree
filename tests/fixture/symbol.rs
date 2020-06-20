#[test]
fn test() {
    let paths = "\
Brewfile
DECISIONS.md
Makefile
README.md
demo.sh
run-tests.sh
scaffold
scaffold/README.md
scaffold/TARGET.cm
scaffold/TARGET.mlb
scaffold/src
scaffold/src/call-main.sml
scaffold/src/main.sig
scaffold/src/main.sml
scaffold/symbol
symbol-new
tests
tests/logging.sh
tests/symbol
tests/symbol/errors.sh
tests/symbol/errors.sh.exp
tests/symbol/help.sh
tests/symbol/help.sh.exp
tests/symbol/infer-with.sh
tests/symbol/overwrite-existing.sh
tests/symbol/with-mlton.sh
tests/symbol/with-mlton.sh.exp
tests/symbol/with-smlnj.sh
tests/symbol/with-smlnj.sh.exp
tests/symbol-new
tests/symbol-new/existing-empty-dot.sh
tests/symbol-new/existing-empty-dot.sh.exp
tests/symbol-new/existing-empty.sh
tests/symbol-new/help.sh
tests/symbol-new/help.sh.exp
tests/symbol-new/infer-with.sh
tests/symbol-new/install-with-smlnj.sh
tests/symbol-new/new-empty.sh
tests/symbol-new/new-empty.sh.exp
tests/symbol-new/no-target.sh
tests/symbol-new/no-target.sh.exp
tests/symbol-new/version.sh
tests/symbol-new/version.sh.exp
tests/travis-install.sh
";
    let expected = "\
.
├── Brewfile
├── DECISIONS.md
├── Makefile
├── README.md
├── demo.sh
├── run-tests.sh
├── scaffold
│   ├── README.md
│   ├── TARGET.cm
│   ├── TARGET.mlb
│   ├── src
│   │   ├── call-main.sml
│   │   ├── main.sig
│   │   └── main.sml
│   └── symbol
├── symbol-new
└── tests
    ├── logging.sh
    ├── symbol
    │   ├── errors.sh
    │   ├── errors.sh.exp
    │   ├── help.sh
    │   ├── help.sh.exp
    │   ├── infer-with.sh
    │   ├── overwrite-existing.sh
    │   ├── with-mlton.sh
    │   ├── with-mlton.sh.exp
    │   ├── with-smlnj.sh
    │   └── with-smlnj.sh.exp
    ├── symbol-new
    │   ├── existing-empty-dot.sh
    │   ├── existing-empty-dot.sh.exp
    │   ├── existing-empty.sh
    │   ├── help.sh
    │   ├── help.sh.exp
    │   ├── infer-with.sh
    │   ├── install-with-smlnj.sh
    │   ├── new-empty.sh
    │   ├── new-empty.sh.exp
    │   ├── no-target.sh
    │   ├── no-target.sh.exp
    │   ├── version.sh
    │   └── version.sh.exp
    └── travis-install.sh
";
    super::common_test(paths, expected);
}
