from functools import cache
class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        @cache
        def min_distance(w1: int, w2: int) -> int:
            if w1 == len(word1):
                return len(word2) - w2
            if w2 == len(word2):
                return len(word1) - w1
            if word1[w1] == word2[w2]:
                return min_distance(w1 + 1, w2 + 1)
            insert_character = 1 + min_distance(w1 + 1,  w2)
            delete_character = 1 + min_distance(w1, w2 + 1)
            replace_character = 1 + min_distance(w1 + 1, w2 + 1)
            return min(insert_character, delete_character, replace_character)
        return min_distance(0,0)
