#include <gtest/gtest.h>
#include "../header.hpp"

// twoSum
// -----------------------------------------------
TEST(twoSum, test1) {
    std::vector<int> lhs {2,7,11,15};
    std::vector<int> rhs{0,1};
    ASSERT_EQ(twoSum(lhs,9),rhs);
}
TEST(twoSum, test2) {
    std::vector<int> lhs {3,2,4};
    std::vector<int> rhs{1,2};
    ASSERT_EQ(twoSum(lhs,6),rhs);
}
TEST(twoSum, test3) {
    std::vector<int> lhs {3,3};
    std::vector<int> rhs{0,1};
    ASSERT_EQ(twoSum(lhs,6),rhs);
}
// lengthOfLongestSubstring
// -----------------------------------------------
TEST(lengthOfLongestSubstring, test1) {
    ASSERT_EQ(lengthOfLongestSubstring(std::string("abcabcbb")),3);
}
TEST(lengthOfLongestSubstring, test2) {
    ASSERT_EQ(lengthOfLongestSubstring(std::string("bbbbb")),1);
}
TEST(lengthOfLongestSubstring, test3) {
    ASSERT_EQ(lengthOfLongestSubstring(std::string("pwwkew")),3);
}
TEST(lengthOfLongestSubstring, test4) {
    ASSERT_EQ(lengthOfLongestSubstring(std::string("abba")),2);
}
TEST(lengthOfLongestSubstring, test5) {
    ASSERT_EQ(lengthOfLongestSubstring(std::string(" ")),1);
}
TEST(lengthOfLongestSubstring, test6) {
    ASSERT_EQ(lengthOfLongestSubstring(std::string("")),0);
}