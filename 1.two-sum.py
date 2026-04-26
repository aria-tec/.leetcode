#
# @lc app=leetcode id=1 lang=python3
#
# [1] Two Sum
#

# @lc code=start
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        h = {}
        for i in range(len(nums)):
            h[nums[i]] = i

        for i in range(len(nums)):
            diff = target - nums[i] 
            if diff in nums and h[diff] != i:
                return [i, h[diff]]
# @lc code=end

