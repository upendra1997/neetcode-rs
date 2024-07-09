from typing import List
from unittest import TestCase

class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        res = 0
        curMin, curMax = 1, 1
        for i in nums:
            if i == 0:
                curMin, curMax = 1, 1
                continue
            tmp = i * curMax
            curMax = max(i * curMax, i * curMin, i)
            curMin = min(curMin * i, tmp, i)
            res = max(res, curMax)
        return res

class TestMe(TestCase):
    def test_me(self):
        sol = Solution()
        res = sol.maxProduct([2,3,-2,4])
        self.assertEqual(res, 6)
 
    def test_me1(self):
        sol = Solution()
        res = sol.maxProduct([-2,0,-1])
        self.assertEqual(res, 0)