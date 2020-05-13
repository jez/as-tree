load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "as-tree",
    crate_root = "main.rs",
    srcs = glob(["*.rs"]),
    deps = [
        "//third_party/cargo:ansi_term",
        "//third_party/cargo:lscolors",
    ],
    visibility = ["//visibility:public"],
)
