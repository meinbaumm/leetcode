# https://leetcode.com/problems/running-sum-of-1d-array/submissions/
# Дан массив. Вывести новый массив, где каждый элемент будет равен сумме стоящих до него элментов

mama = [1, 1, 1, 1]

def runningSum(nums):
    next = 0
    next_list = []
    if len(nums) <= 2:
        return sum(nums)
    else:
        for i in range(1, len(nums) + 1):
            next = sum(nums[:i])
            next_list.append(next)
        return next_list

if __name__ == '__main__':
    print(runningSum(mama)) # [1, 2, 3, 4]


# Прекрасное решение из комментариев
def runningSum(nums):
    for i in range(1, len(nums)):
        nums[i] += nums[i - 1]
    return nums

if __name__ == '__main__':
    print(runningSum(mama))