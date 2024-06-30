from collections import defaultdict
from typing import List

class Solution:
    def solve(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        visited = defaultdict(lambda: False)
        deltas = [(0, -1), (0, 1), (1, 0), (-1, 0)]
        def dfs(x: int, y: int):
            if visited[(x,y)]:
                return
            if board[x][y] == 'X':
                return
            visited[(x,y)] = True
            for (dx, dy) in deltas:
                new_x = x + dx
                new_y = y + dy
                if new_x < 0 or new_x >= len(board):
                    continue
                if new_y < 0 or new_y >= len(board[0]):
                    continue
                dfs(new_x, new_y)

        for (r, row) in enumerate(board):
            for (c, col) in enumerate(row):
                if not visited[(r,c)] and col == 'O':
                    surrounded = True
                    dfs(r,c)
                    for (k, v) in visited.items():
                        if v == False:
                            continue
                        x,y = k
                        if x == 0 or x == len(board) - 1:
                            surrounded = False
                            break
                        if y == 0 or y == len(board[0]) - 1:
                            surrounded = False
                            break
                    if surrounded:
                        for (k, v) in visited.items():
                            x,y = k
                            board[x][y] = 'X'
                    visited = defaultdict(lambda: False)
