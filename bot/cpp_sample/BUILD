# -*- mode: python -*-

cc_binary(
  name = "cpp_sample",
  srcs = [
      "value.h",
      "framework.h",
      "game.h",
      "main.cpp",
  ],
)

cc_test(
  name = "cpp_sample_test",
  srcs = [
      "value.h",
      "framework.h",
      "game.h",
      "test.cpp",
  ],
  deps = [
      "@com_github_google_googletest//:gtest",
      "@com_github_google_googletest//:gtest_main",
  ],
)
