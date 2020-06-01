# -*- mode: python -*-

# Workspace documentation:
# https://docs.bazel.build/versions/master/be/workspace.html

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Abseil (https://abseil.io/)
#
# Mainly for string functions.
# https://abseil.io/docs/cpp/guides/strings
# https://abseil.io/docs/cpp/guides/format
http_archive(
    name = "com_google_absl",
    sha256 = "f342aac71a62861ac784cadb8127d5a42c6c61ab1cd07f00aef05f2cc4988c42",
    strip_prefix = "abseil-cpp-20200225.2",
    urls = ["https://github.com/abseil/abseil-cpp/archive/20200225.2.zip"],
)

# gflags (https://gflags.github.io/gflags/)
http_archive(
    name = "com_github_gflags_gflags",
    sha256 = "19713a36c9f32b33df59d1c79b4958434cb005b5b47dc5400a7a4b078111d9b5",
    strip_prefix = "gflags-2.2.2",
    urls = ["https://github.com/gflags/gflags/archive/v2.2.2.zip"],
)

# glog (https://github.com/google/glog)
http_archive(
    name = "com_github_google_glog",
    sha256 = "e94a39c4ac6fab6fdf75b37201e0333dce7fbd996e3f9c4337136ea2ecb634fc",
    strip_prefix = "glog-6ca3d3cf5878020ebed7239139d6cd229a1e7edb",
    urls = ["https://github.com/google/glog/archive/6ca3d3cf5878020ebed7239139d6cd229a1e7edb.zip"],  # 2019-04-12
)

# gtest (https://github.com/google/googletest)
http_archive(
    name = "com_github_google_googletest",
    sha256 = "94c634d499558a76fa649edb13721dce6e98fb1e7018dfaeba3cd7a083945e91",
    strip_prefix = "googletest-release-1.10.0",
    urls = ["https://github.com/google/googletest/archive/release-1.10.0.zip"],
)

# Protocol Buffers (https://github.com/protocolbuffers/protobuf)
http_archive(
    name = "com_google_protobuf",
    sha256 = "cf754718b0aa945b00550ed7962ddc167167bd922b842199eeb6505e6f344852",
    strip_prefix = "protobuf-3.11.3",
    urls = [
        "https://mirror.bazel.build/github.com/protocolbuffers/protobuf/archive/v3.11.3.tar.gz",
        "https://github.com/protocolbuffers/protobuf/archive/v3.11.3.tar.gz",
    ],
)

# Default dependencies.
load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
protobuf_deps()
