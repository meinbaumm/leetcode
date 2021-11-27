# https://leetcode.com/problems/jewels-and-stones/
# 771. Jewels and Stones

def numJewelsInStones(jewels, stones):
    jewels_in_stones = 0
    for j in stones:
        if j in jewels:
            jewels_in_stones += 1
    return jewels_in_stones


# Or functional
def numJewelsInStones(jewels, stones):
    def compounder(x):
        if x in jewels:
            return 1
        return 0
    return sum(map(compounder, stones))

jewels = "z"
stones = "ZZ"


if __name__ == '__main__':
    print(numJewelsInStones(jewels, stones))