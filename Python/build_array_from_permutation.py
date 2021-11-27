# https://leetcode.com/problems/build-array-from-permutation/
# Build Array from Permutation where ans = [nums[nums[0]], nums[nums[1]], nums[nums[2]], nums[nums[3]], nums[nums[4]], nums[nums[5]]]

def buildArray(nums):
    ans = []
    for ind in range(len(nums)):
        ans.append(nums[nums[ind]])
    return ans


# Or much faster
def buildArray(nums):
    return [nums[nums[i]] for i in range(len(nums))]


nums = [5,0,1,2,3,4]


if __name__ == '__main__':
    print(buildArray(nums))  # [4,5,0,1,2,3]