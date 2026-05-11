#
# @lc app=leetcode id=217 lang=python3
#
# [217] Contains Duplicate
#

# @lc code=start
class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        # Convert the array into a hash set, which removes duplicates.
        set_nums = set(nums)

        # Compare the size of the set with the size of the original array.
        # If the set is smaller, return True, because duplicates must have been removed.
        if len(set_nums) < len(nums):
            return True

        # Otherwise, return False.
        return False


# @lc code=end

