from functools import lru_cache

class Solution:
    def climbStairs(self, n: int) -> int:
        @lru_cache(maxsize=99999)
        def climb(current):
            if current == n:
                return 1
            if current > n:
                return 0
            one = climb(current + 1)
            two = climb(current + 2)
            return one + two
        return climb(0)
