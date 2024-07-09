from bisect import bisect
from typing import List

import unittest
class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        def join(left, right):
            l_x, l_y = left
            r_x, r_y = right
            if l_x > r_x:
                l_x, l_y = right
                r_x, r_y = left
            if l_y < r_x:
                return None
            else:
                return (min(l_x, r_x), max(l_y, r_y))
        
        interval = sorted(map(lambda x: (x[0], x[1]), intervals))

        result = []
        i = 0
        while i < len(interval):
            ress = None
            res = interval[i]
            while i < len(interval):
                res = join(interval[i], res)
                i += 1
                if res == None:
                    i -= 1
                    break
                else:
                    ress = res
            if ress:
                result.append(ress)
            else:
                if i < len(interval):
                    result.append(interval[i])
                    i += 1

        return list(map(lambda x: [x[0], x[1]], result))

class Test(unittest.TestCase):
    def test(self):
        intervals = [[1,3],[2,6],[8,10],[15,18]]
        output = [[1,6],[8,10],[15,18]]
        self.assertEqual(Solution().merge(intervals), output)
    def test1(self):
        intervals = [[1,4],[4,5]]
        output = [[1, 5]]
        self.assertEqual(Solution().merge(intervals), output)