from functools import cache
class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        @cache
        def search(idx: int) -> int:
            if idx >= len(nums):
                return 0
            res = 1
            for i in range(idx+1, len(nums)):
                if nums[idx] < nums[i]:
                    res = max(res, 1 + search(i))
            return res
        return max((search(i) for i in range(len(nums))))