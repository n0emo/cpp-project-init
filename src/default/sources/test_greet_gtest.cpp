#include "lib.hpp"

#include <gtest/gtest.h>

TEST(HelloTest, BasicAssertions) {
  EXPECT_EQ(greet("World"), "Hello, World!");
}
