# https://leetcode.com/problems/concatenation-of-array/
# Concatenation of Array return [arr, arr]

def getConcatenation(nums):
    for ind in range(len(nums)):
        nums.append(nums[ind])
    return nums

# Or
def getConcatenation(nums):
    return nums + nums

# Or 
def getConcatenation(nums):
    return nums * 2


nums = [1, 2, 3]


if __name__ == '__main__':
    print(getConcatenation(nums))