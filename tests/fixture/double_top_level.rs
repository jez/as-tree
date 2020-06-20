#[test]
fn test() {
    let paths = "\
top1/dir1
top1/dir1/bar.txt
top1/dir1/foo.txt
top1/dir2
top1/dir2/qux.txt
top1/run.sh
top1/run.sh.exp
top2/dir1
top2/dir1/bar.txt
top2/dir1/foo.txt
top2/dir2
top2/dir2/qux.txt
top2/run.sh
top2/run.sh.exp
";
    let expected = "\
.
├── top1
│   ├── dir1
│   │   ├── bar.txt
│   │   └── foo.txt
│   ├── dir2
│   │   └── qux.txt
│   ├── run.sh
│   └── run.sh.exp
└── top2
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
