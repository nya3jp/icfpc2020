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

# CPR (https://github.com/whoshuu/cpr)
http_archive(
    name = "cpr",
    build_file = "//third_party:cpr.BUILD",
    sha256 = "e4805a5897acceab4c159f5dd47b92f9bbfc91bd984eed29af8b69ec6c368052",
    strip_prefix = "cpr-1.5.0",
    urls = [
        "https://github.com/whoshuu/cpr/archive/1.5.0.zip",
    ],
)

# libcurl (https://curl.haxx.se/libcurl/)
http_archive(
    name = "curl",
    build_file = "//third_party:curl.BUILD",
    sha256 = "01ae0c123dee45b01bbaef94c0bc00ed2aec89cb2ee0fd598e0d302a6b5e0a98",
    strip_prefix = "curl-7.69.1",
    urls = [
        "https://storage.googleapis.com/mirror.tensorflow.org/curl.haxx.se/download/curl-7.69.1.tar.gz",
        "https://curl.haxx.se/download/curl-7.69.1.tar.gz",
    ],
)

# zlib (https://zlib.net/)
http_archive(
    name = "zlib",
    build_file = "//third_party:zlib.BUILD",
    sha256 = "c3e5e9fdd5004dcb542feda5ee4f0ff0744628baf8ed2dd5d66f8ca1197cb1a1",
    strip_prefix = "zlib-1.2.11",
    urls = [
        "https://storage.googleapis.com/mirror.tensorflow.org/zlib.net/zlib-1.2.11.tar.gz",
        "https://zlib.net/zlib-1.2.11.tar.gz",
    ],
)
