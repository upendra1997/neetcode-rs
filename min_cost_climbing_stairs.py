from functools import lru_cache
from typing import List

class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        @lru_cache(maxsize=99999)
        def climb(current):
            if current == len(cost):
                return 0
            if current > len(cost):
                return 999999999
            one = climb(current + 1)
            two = climb(current + 2)
            return cost[current] + min(one, two)
        return min(climb(0), climb(1))
