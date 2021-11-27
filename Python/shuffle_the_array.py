# https://leetcode.com/problems/shuffle-the-array/
# 1470. Shuffle the Array

def shuffle(nums, n):
    ans = []
    idx, jdx = 0, n
    
    while jdx < len(nums):
        
        ans.append(nums[idx])
        idx += 1
        
        ans.append(nums[jdx])
        jdx += 1
    return ans


nums = [2,5,1,3,4,7]
n = 3


if __name__ == '__main__':
    print(shuffle(nums, n)) 
