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
