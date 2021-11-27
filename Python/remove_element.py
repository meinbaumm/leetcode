# https://leetcode.com/problems/remove-element
# Remove all values from array and return len of new array

def removeElement(nums, val):
    if len(nums) == 0:
        return 0
    i = len(nums) - 1
    while i >= 0:
        if nums[i] is val:
            nums.pop(i)
            i = len(nums)
        i -= 1
    return len(nums)


nums = [0,1,2,2,3,0,4,2]
val = 2


if __name__ == '__main__':
    print(removeElement(nums, val))  # 5