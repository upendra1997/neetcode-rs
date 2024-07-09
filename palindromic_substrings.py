from typing import Tuple, Generator

class Solution:
    def countSubstrings(self, s: str) -> int:
        def palindrom_length(start: int, end: int) -> Generator[Tuple[int, str], None, None]:
            while start >= 0 and end < len(s) and s[start] == s[end]:
                yield (end - start + 1, s[start: end + 1])
                start -= 1
                end += 1
            return None
        r = 0
        for i in range(len(s)):
            for _ in palindrom_length(i, i):
                r += 1
            for _ in palindrom_length(i, i + 1):
                r += 1
        return r
        
