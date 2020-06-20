mod as_tree_absolute;
mod as_tree_simple;
mod collapse;
mod double_top_level;
mod empty;
mod find_dot;
mod sorbet;
mod sorbet_extension_c_h_cc_hh;
mod sorbet_extension_md;
mod sorbet_shuf;
mod symbol;

fn common_test(paths: &str, expected: &str) {
    let trie: as_tree::PathTrie = paths.lines().collect();
    let result = format!("{}", trie.display());
    #[cfg(not(target_os = "windows"))]
    pretty_assertions::assert_eq!(result, expected);
    // stack overflow when comparing two very long buffers on Windows
    #[cfg(target_os = "windows")]
    for (result_line, expected_line) in result.lines().zip(expected.lines()) {
        pretty_assertions::assert_eq!(result_line, expected_line)
    }
}
