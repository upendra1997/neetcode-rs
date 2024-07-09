from functools import cache
class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        @cache
        def path(r: int, c: int) -> int:
            if r >= m:
                return 0
            if c >= n:
                return 0
            if r == m - 1 and c == n - 1:
                return 1
            result = path(r + 1, c) + path(r, c + 1)
            return result
        return path(0,0)