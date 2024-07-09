from bisect import bisect
from typing import List

import unittest
class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
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

        new_interval = (newInterval[0], newInterval[1])
        index = bisect(interval, new_interval)
        interval.insert(index, new_interval)
        
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
        intervals = [[1,3],[6,9]]
        newInterval = [2,5]
        output = [[1,5],[6,9]]
        self.assertEqual(Solution().insert(intervals, newInterval), output)
    def test1(self):
        intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]]
        newInterval = [4,8]
        output = [[1,2],[3,10],[12,16]]
        self.assertEqual(Solution().insert(intervals, newInterval), output)