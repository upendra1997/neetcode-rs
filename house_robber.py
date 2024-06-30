from typing import List
from functools import lru_cache

class Solution:
    def rob(self, nums: List[int]) -> int:
        @lru_cache(maxsize=999999)
        def rob_me(i):
            if i >= len(nums):
                return 0
            r = nums[i] + rob_me(i + 2)
            nr = rob_me(i + 1)
            return max(r ,nr)
        res = rob_me(0)
        return res
