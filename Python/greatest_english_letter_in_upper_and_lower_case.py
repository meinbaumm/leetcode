# https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/
# 2309. Greatest English Letter in Upper and Lower Case

def greatestLetter(s: str) -> str:
        s = set(s)
        upper, lower = ord('Z'), ord('z')
        for i in range(26):
            if chr(upper - i) in s and chr(lower - i) in s:
                return chr(upper - i)
        return ''



s1 = "lEeTcOdE"
s2 = "arRAzFif"
s3 = "AbCdEfGhIjK"

print(greatestLetter(s1))
print(greatestLetter(s2))
print(greatestLetter(s3))
