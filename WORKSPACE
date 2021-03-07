BAZEL_VERSION = "3.1.0"

workspace(name = "io_jez_rs_as_tree")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "com_grail_bazel_toolchain",
    url = "https://github.com/grailbio/bazel-toolchain/archive/f2d1ba2c9d713b2aa6e7063f6d11dd3d64aa402a.zip",
    sha256 = "0246482b21a04667825c655d3b4f8f796d842817b2e11f536bbfed5673cbfd97",
    strip_prefix = "bazel-toolchain-f2d1ba2c9d713b2aa6e7063f6d11dd3d64aa402a",
)

load("@com_grail_bazel_toolchain//toolchain:deps.bzl", "bazel_toolchain_dependencies")
bazel_toolchain_dependencies()

load("@com_grail_bazel_toolchain//toolchain:rules.bzl", "llvm_toolchain")
llvm_toolchain(
    name = "llvm_toolchain",
    absolute_paths = True,
    llvm_version = "9.0.0",
)

load("@llvm_toolchain//:toolchains.bzl", "llvm_register_toolchains")
llvm_register_toolchains()

# rules_rust (or maybe Bazel itself?)
http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "800ffbce5af3f196448b4844b8ad32f82f6ff1cda192ebf5edd5f5a9d132f348",
    strip_prefix = "rules_rust-6835a3c8ed1dcd67040cccd603ff3daf611ce41c",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/6835a3c8ed1dcd67040cccd603ff3daf611ce41c.zip",
    ],
)

http_archive(
    name = "bazel_skylib",
    sha256 = "9a737999532daca978a158f94e77e9af6a6a169709c0cee274f0a4c3359519bd",
    strip_prefix = "bazel-skylib-1.0.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/1.0.0.tar.gz",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()

BAZEL_INSTALLER_VERSION_darwin_SHA = "5cfa97031b43432b3c742c80e2e01c41c0acdca7ba1052fc8cf1e291271bc9cd"
BAZEL_INSTALLER_VERSION_linux_SHA = "7ba815cbac712d061fe728fef958651512ff394b2708e89f79586ec93d1185ed"
