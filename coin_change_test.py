from functools import cache
class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        @cache
        def chng(a, idx):
            if a == amount:
                return 1
            if a > amount:
                return 0
            if idx == len(coins):
                return 0
            result = chng(a + coins[idx], idx) + chng(a, idx + 1)
            return result
        return chng(0, 0)