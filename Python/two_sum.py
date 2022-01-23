# https://leetcode.com/problems/two-sum/submissions/

nums = [1, 3, 1, 3, 3, 3, 2]

# Указываешь массив и цель в массиве 
# Цель в массиве – необходимая получаемая сумма из двух чесел[индексов] в массиве
def twoSum(nums, target): # Выводит два индекса, числа которых в сумме доют цель 
    seen = {} 
    for i, v in enumerate(nums):
        remaining = target - v
        if remaining in seen:
            return [seen[remaining], i]
        seen[v] = i
    return []

print(twoSum(nums=nums, target=5))
