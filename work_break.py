from typing import List
from unittest import TestCase
from functools import cache

class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        @cache
        def search(s: str) -> bool:
            if s == "":
                return True
            result = False
            for k in wordDict:
                if s.startswith(k):
                    result = result or search(s[len(k):])
            return result
        return search(s)

class Testing(TestCase):
    def testing_hello(self):
        s = "leetcode"
        wordDict = ["leet", "code"]
        res = True
        self.assertEqual(Solution().wordBreak(s, wordDict), res)
    def testing_hx(self):
        s = "applepenapple"
        wordDict = ["apple", "pen"]
        res = True
        self.assertEqual(Solution().wordBreak(s, wordDict), res)
    def testing_h(self):
        s = "catsandog"
        wordDict = ["cats","dog","sand","and","cat"]
        res = False
        self.assertEqual(Solution().wordBreak(s, wordDict), res)