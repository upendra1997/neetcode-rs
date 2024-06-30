from typing import List

class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        parent = {x: x for x in range(1, len(edges) + 1)}
        rank = {x: 1 for x in range(1, len(edges) + 1)}
            
        def find(x):
            p = parent[x]
            while p != parent[p]:
                parent[p] = parent[parent[p]]
                p = parent[p]
            return p

        def union(x: int, y: int):
            px, py = find(x), find(y)
            if px == py:
                return False
            if rank[x] > rank[y]:
                parent[py] = parent[px]
                rank[px] += rank[py]
            else:
                parent[px] = parent[py]
                rank[py] += rank[px]
            return True

        for edge in edges:
            if not union(edge[0], edge[1]):
                return edge
        return [0,0]
