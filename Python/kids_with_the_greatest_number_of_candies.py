# https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
# 1431. Kids With the Greatest Number of Candies

def kidsWithCandies(candies, extraCandies):
    max_ = max(candies) - extraCandies
    return [x >= max_ for x in candies]


candies = [4, 2, 1, 1, 2]
extraCandies = 1


if __name__ == '__main__':
    print(kidsWithCandies(candies, extraCandies))  # [True, False, False, False, False]