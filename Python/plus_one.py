# https://leetcode.com/problems/plus-one/
# 66. Plus One
from typing import List

class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        if digits[-1] != 9:
            digits[-1] += 1
        else:
            if len(digits) <= 1:
                digits = [1,0]
            else:
                digits = self.plusOne(digits[:-1]) + [0]
        return digits


digits1 = [1,2,3]
digits2 = [9]
digits3 = [9,9]

print(Solution().plusOne(digits1))
print(Solution().plusOne([0]))
print(Solution().plusOne(digits3))
