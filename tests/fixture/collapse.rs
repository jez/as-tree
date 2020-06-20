#[test]
fn test() {
    let paths = "\
test/cli/color/dir1
test/cli/color/dir1/bar.txt
test/cli/color/dir1/foo.txt
test/cli/color/dir2
test/cli/color/dir2/qux.txt
test/cli/color/run.sh
test/cli/color/run.sh.exp
";
    #[cfg(target_os = "windows")]
    let expected = "\
test\\cli\\color
├── dir1
│   ├── bar.txt
│   └── foo.txt
├── dir2
│   └── qux.txt
├── run.sh
└── run.sh.exp
";
    #[cfg(not(target_os = "windows"))]
    let expected = "\
test/cli/color
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
