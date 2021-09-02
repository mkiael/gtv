#include <gtest/gtest.h>

TEST(FirstTest, ShouldPass)
{
   EXPECT_EQ(1, 1);
}

TEST(SecondTest, ShouldAlsoPass)
{
   EXPECT_EQ(1, 1);
}

TEST(ThirdTest, ShouldFail)
{
   EXPECT_EQ(2, 1);
}

TEST(FourthTest, YetAnotherPassingTest)
{
   EXPECT_EQ(1, 1);
}

TEST(FifthTest, OneMoreFail)
{
   EXPECT_EQ(1, 2);
}
