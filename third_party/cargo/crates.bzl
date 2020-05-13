"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        new_git_repository(name=name, **kwargs)

def raze_fetch_remote_crates():

    _new_http_archive(
        name = "raze__ansi_term__0_12_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ansi_term/ansi_term-0.12.1.crate",
        type = "tar.gz",
        strip_prefix = "ansi_term-0.12.1",
        sha256 = "d52a9bb7ec0cf484c551830a7ce27bd20d67eac647e1befb56b0be4ee39a55d2",

        build_file = Label("//third_party/cargo/remote:ansi_term-0.12.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__atty__0_2_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.14.crate",
        type = "tar.gz",
        strip_prefix = "atty-0.2.14",
        sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8",

        build_file = Label("//third_party/cargo/remote:atty-0.2.14.BUILD"),
    )

    _new_http_archive(
        name = "raze__hermit_abi__0_1_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hermit-abi/hermit-abi-0.1.12.crate",
        type = "tar.gz",
        strip_prefix = "hermit-abi-0.1.12",

        build_file = Label("//third_party/cargo/remote:hermit-abi-0.1.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__libc__0_2_70",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.70.crate",
        type = "tar.gz",
        strip_prefix = "libc-0.2.70",
        sha256 = "3baa92041a6fec78c687fa0cc2b3fae8884f743d672cf551bed1d6dac6988d0f",

        build_file = Label("//third_party/cargo/remote:libc-0.2.70.BUILD"),
    )

    _new_http_archive(
        name = "raze__lscolors__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lscolors/lscolors-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "lscolors-0.7.0",
        sha256 = "1f77452267149eac960ded529fe5f5460ddf792845a1d71b5d0cfcee5642e47e",

        build_file = Label("//third_party/cargo/remote:lscolors-0.7.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.8.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.3.8",

        build_file = Label("//third_party/cargo/remote:winapi-0.3.8.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",

        build_file = Label("//third_party/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",

        build_file = Label("//third_party/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"),
    )

