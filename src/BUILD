load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "as-tree",
    srcs = ["main.rs"],
    deps = [
        "//third_party/cargo:ansi_term",
        "//third_party/cargo:lscolors",
    ],
    visibility = ["//visibility:public"],
)
