# https://leetcode.com/problems/richest-customer-wealth/
# Richest Customer Wealth


def maximumWealth(accounts):
    max_wealth = 0
    for customer in accounts:
        customer_sum = sum(customer)
        if customer_sum > max_wealth:
            max_wealth = customer_sum
    return max_wealth


# Or faster
def maximumWealth(accounts):
    return max(sum(i) for i in accounts)


accounts = [[1,5],[7,3],[3,5]]


if __name__ == '__main__':
    print(maximumWealth(accounts))  # 10