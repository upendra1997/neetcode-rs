import unittest
from typing import Tuple, Generator

class Solution:
    def longestPalindrome(self, s: str) -> str:
        def palindrom_length(start: int, end: int) -> Generator[Tuple[int, str], None, None]:
            while start >= 0 and end < len(s) and s[start] == s[end]:
                yield (end - start + 1, s[start: end + 1])
                start -= 1
                end += 1
            return None
        res = 0
        r = ""
        for i in range(len(s)):
            for (l, r_) in palindrom_length(i, i):
                if l > res:
                    res = l
                    r = r_
            for (l, r_) in palindrom_length(i, i + 1):
                if l > res:
                    res = l
                    r = r_
        return r
            



class TestPalindrome(unittest.TestCase):
    def test_something(self):
       self.assertEqual(Solution().longestPalindrome("BABAD"), "BAB")
    def test_something_else(self):
       self.assertEqual(Solution().longestPalindrome("CBBD"), "BB")
    def test_whatever(self):
       self.assertEqual(Solution().longestPalindrome("aacabdkacaa"), "aca")
        
