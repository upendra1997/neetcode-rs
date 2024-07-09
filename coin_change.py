from typing import List
from functools import cache
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        @cache
        def dp(a: int) -> float:
            if a < 0:
                return float('inf')
            if a == 0:
                return 0
            else:
                res = float('inf')
                for c in coins:
                    res = min(res, 1 + dp(a - c))
            return res
        res = dp(amount)
        if res == float('inf'):
            return -1
        else:
            return int(res)
