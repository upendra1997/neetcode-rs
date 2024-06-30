from sys import maxsize
from types import new_class
from typing import List
from collections import defaultdict, deque
from functools import lru_cache
from enum import Enum


class State(Enum):
    NotVisited = 0
    Visited = 1
    Processing = 2

class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:

        def get_wildcard(w):
            for c in range(len(w)):
                yield "{}*{}".format(w[:c], w[c+1:])
            return

        words = set(wordList)
        graph = defaultdict(list)
        for w in words:
            for new_word in get_wildcard(w):
                graph[new_word].append(w)
        visited = defaultdict(lambda: State.NotVisited)
        distance = defaultdict(lambda: 9999999)
        parent = defaultdict(lambda: None)

        @lru_cache(maxsize=5000*5000)
        def diff_words(word1: str, word2: str) -> int:
            count = 0
            for (c1, c2) in zip(word1, word2):
                if c1 != c2:
                    count += 1
            return count

        distance[beginWord] = 0
        parent[beginWord] = None
        
        q = deque()
        q.append(beginWord)

        while len(q):
            e = q.popleft()
            visited[e] = State.Processing
            for wild in get_wildcard(e):
                for word in graph[wild]:
                    if visited[word] == State.NotVisited:
                        new_distance = distance[e] + diff_words(e, word)
                        if new_distance < distance[word]:
                            parent[word] = e
                            distance[word] = new_distance
                        q.append(word)
            visited[e] = State.Visited
        if distance[endWord] ==  9999999:
            return 0
        return distance[endWord] + 1
