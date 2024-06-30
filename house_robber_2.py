from typing import List
from functools import lru_cache

class Solution:
    def rob(self, nums: List[int]) -> int:
        @lru_cache(maxsize=999999)
        def rob_me(i, n):
            if i >= n:
                return 0
            r = nums[i] + rob_me(i + 2, n)
            nr = rob_me(i + 1, n)
            return max(r ,nr)

        if len(nums) == 1:
            return nums[0]
        res = max(rob_me(0, len(nums) - 1), rob_me(1, len(nums)))
        return res
