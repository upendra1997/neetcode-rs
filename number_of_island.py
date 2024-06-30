from collections import defaultdict
from typing import Tuple
from logging import Logger
import unittest


class Solution:
    def numIslands(self, grid: list[list[str]]) -> int:
        visited = defaultdict(lambda: False)
        def dfs(start: Tuple[int, int]):
            if visited[start]:
                return
            x,y = start
            if grid[x][y] == '0':
                return
            visited[start] = True
            deltas = [(0,1), (0, -1), (1,0), (-1,0)]
            for (dx,dy) in deltas:
                new_x = x + dx
                new_y = y + dy
                if new_x >= len(grid) or new_x < 0:
                    continue
                if new_y >= len(grid[0]) or new_y < 0:
                    continue
                dfs((new_x, new_y))
        count = 0
        for x in range(len(grid)):
            for y in range(len(grid[0])):
                t = (x, y)
                if not visited[t] and grid[x][y] == '1':
                    # print("finding islands here", t)
                    # logger.warning("finding islands here", t)
                    count += 1
                    dfs(t)
        # logger.warning(visited)
        return count


class TestSolution(unittest.TestCase):
    def test_num_islands(self):
        grid = [ ["1","1","0","0","0"], ["1","1","0","0","0"], ["0","0","1","0","0"], ["0","0","0","1","1"] ]
        sol = Solution()
        res = sol.numIslands(grid)
        self.assertEqual(res, 3)
