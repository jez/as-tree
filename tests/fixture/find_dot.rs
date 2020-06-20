#[test]
fn test() {
    let paths = "\
.
./dir2
./dir2/qux.txt
./run.sh.exp
./run.sh
./dir1
./dir1/foo.txt
./dir1/bar.txt
";
    let expected = "\
.
├── dir1
│   ├── bar.txt
│   └── foo.txt
├── dir2
│   └── qux.txt
├── run.sh
└── run.sh.exp
";
    super::common_test(paths, expected);
}
