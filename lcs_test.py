from functools import cache
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        @cache
        def lcs(idx1: int, idx2: int) -> int:
            if idx1 == len(text1):
                return 0
            if idx2 == len(text2):
                return 0
            if text1[idx1] == text2[idx2]:
                return 1 + lcs(idx1 + 1, idx2 + 1)
            return max(lcs(idx1 + 1, idx2), lcs(idx1, idx2 + 1))
        return lcs(0 ,0)