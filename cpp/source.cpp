#include "header.hpp"

// Runtime: 0 ms, faster than 100.00%
// Memory Usage: 6.9 MB, less than 96.34%
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
int lengthOfLongestSubstring(std::string s) {
    int map[256];
    std::fill_n(map,256,-1);
    int start = 0;
    int max = 0;
    for(int i=0;i<s.size();i++) {
        int& index = map[s[i]];
        if (index != -1) {
            if (index >= start) {
                max = std::max(max,i-start);
                start = index+1;
            }
            index = i;
        }
        else {
            map[s[i]] = i;
        }
    }
    return std::max(max,(int)s.size()-start);
}
// https://leetcode.com/problems/two-sum/
std::vector<int> twoSum(std::vector<int>& nums, int target) {
    std::unordered_map<int,int> map;
    for(int i = 0; i<nums.size();i++) {
        int diff = target-nums[i];
        // If map contains required difference
        if(map.find(diff) != map.end()) {
            return std::vector<int>{map[diff],i};
        }
        // Otherwise add this value to our map under this index
        map[nums[i]]=i;
    }
    return std::vector<int>{};
}