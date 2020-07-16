# -*- mode: python -*-

# Rules documentation:
# https://docs.bazel.build/versions/master/be/overview.html#rules
load("@io_bazel_rules_go//go:def.bzl", "go_test")

cc_binary(
    name = "app",
    srcs = ["app.cc"],
    deps = ["@cpr"],
)

cc_binary(
    name = "minimal_main",
    srcs = [
        "minimal_main.cc",
    ],
    deps = [
        ":minimal_lib",
        "@com_github_gflags_gflags//:gflags",
        "@com_github_google_glog//:glog",
    ],
)

cc_library(
    name = "minimal_lib",
    srcs = [
        "minimal_lib.cc",
    ],
    hdrs = [
        "minimal_lib.h",
    ],
    deps = [
        "@com_google_absl//absl/strings:str_format",
    ],
)

cc_test(
    name = "minimal_lib_test",
    srcs = [
        "minimal_lib_test.cc",
    ],
    deps = [
        ":minimal_lib",
        "@com_github_google_googletest//:gtest",
        "@com_github_google_googletest//:gtest_main",
    ],
)

cc_binary(
    name = "protobuf",
    srcs = [
        "protobuf.cc",
    ],
    deps = [
        ":data_cc_proto",
    ],
)

proto_library(
    name = "data_proto",
    srcs = ["data.proto"],
)

cc_proto_library(
    name = "data_cc_proto",
    deps = [":data_proto"],
)

go_test(
    name = "e2e_test",
    srcs = ["e2e_test.go"],
    args = ["--test_binary", "$(rootpath :app)"],
    data = [":app"],
)
