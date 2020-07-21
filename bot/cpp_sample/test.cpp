// Copyright 2020 Google LLC
// Copyright 2020 Team Spacecat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "gtest/gtest.h"

#include "value.h"

TEST(ModulateTest, Cons) {
  Value v1(1);
  Value v2(81740);
  Value nil;
  Value inner(&v2, &nil);
  Value l(&v1, &inner);

  EXPECT_EQ(modulate(&l), "110110000111011111100001001111110100110000");
  EXPECT_EQ(modulate(&v1), "01100001");
}

TEST(ModulateTest, Negative) {
  Value m1(-1);
  EXPECT_EQ(modulate(&m1), "10100001");
}

TEST(DemodulateTest, List) {
  std::string y = "110110000111011111100001001111110100110000";
  std::string x = "(1 81740)";
  EXPECT_EQ(demodulate(y).to_string(), x);
}

TEST(DemodulateTest, Zero) {
  std::string y = "010";
  std::string x = "0";
  EXPECT_EQ(demodulate(y).to_string(), x);
}

TEST(DemodulateTest, One) {
  std::string y = "01100001";
  std::string x = "1";
  EXPECT_EQ(demodulate(y).to_string(), x);
}

TEST(DemodulateTest, MinusOne) {
  std::string y = "10100001";
  std::string x = "-1";
  EXPECT_EQ(demodulate(y).to_string(), x);
}
