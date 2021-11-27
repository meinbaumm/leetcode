# https://leetcode.com/problems/remove-duplicates-from-sorted-array/
# Remove Duplicates from Sorted Array and return len of new array


def removeDuplicates(nums):
    return len(set(nums))



nums = [0,0,1,1,1,2,2,3,3,4]
# nums = [1, 1, 2]


if __name__ == '__main__':
    print(removeDuplicates(nums))