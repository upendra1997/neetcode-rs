from functools import cache
class Solution:
    def numDecodings(self, s: str) -> int:
        @cache
        def numDecode(s) -> int:
            if len(s) == 0:
                return 1
            if s[0] == '0':
                    return 0
            if len(s) == 1:    
                return 1
            two_char = int(s[:2])
            extra_count = 0
            if two_char <= 26 and two_char > 9:
                extra_count += numDecode(s[2:])
            return extra_count + numDecode(s[1:])
        return numDecode(s)
