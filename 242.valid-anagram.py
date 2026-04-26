"""Module to solve LeetCode Valid Anagram."""

#
# @lc app=leetcode id=242 lang=python3
#
# [242] Valid Anagram
#

# @lc code=start
class Solution:
    """Solution for valid anagram problem."""

    def isAnagram(self, s: str, t: str) -> bool: # noqa: N802
        """Check if two strings are anagrams of each other."""
        # if the length of the string differ, return False
        if len(s) != len(t):
            return False

        # Create a frequency array count of size 26 initialized to 0
        count = [0] * 26

        # Iterate through both strings
        for i in range(len(s)):
            # increment s[i]
            count[ord(s[i]) - ord("a")] += 1

            # decrement s[i]
            count[ord(t[i]) - ord("a")] -= 1

        # Iterate through count array if value not 0 False if value 0 True
        return all(val == 0 for val in count)
# @lc code=end

