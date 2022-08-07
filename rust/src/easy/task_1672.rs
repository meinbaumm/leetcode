// 1672. Richest Customer Wealth
// https://leetcode.com/problems/richest-customer-wealth/

use crate::Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut richest_customer = 0;
        for customer in accounts.iter() {
            let wealth = customer.iter().sum();
            if let true = wealth > richest_customer {
                richest_customer = wealth;
            }
        }
        richest_customer
    }

    pub fn maximum_wealth_iter(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|v| v.iter().sum()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::maximum_wealth_iter(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::maximum_wealth_iter(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::maximum_wealth_iter(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }
}
