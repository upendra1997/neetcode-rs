from collections import defaultdict
from enum import Enum
from typing import List

class State(Enum):
    NotVisited = 0
    Processing = 1
    Visted = 2

class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        graph = defaultdict(list)
        for edges in prerequisites:
            graph[edges[1]].append(edges[0])

        state = defaultdict(lambda: State.NotVisited)
        result = []
        cycle = False
        def dfs(node):
            nonlocal cycle
            match state[node]:
                case State.Visted:
                    return
                case State.Processing:
                    cycle = True
                    return
            state[node] = State.Processing
            for n in graph[node]:
                dfs(n)
            state[node] = State.Visted
            result.insert(0, node)
        for i in range(numCourses):
            if state[i] == State.NotVisited:
                dfs(i)
        if cycle:
            return []
        return result

