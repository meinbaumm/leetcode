// 169. Majority Element
// https://leetcode.com/problems/majority-element/

use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let max = nums
            .into_iter()
            .fold(HashMap::<i32, usize>::new(), |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            })
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);

        if let Some(val) = max {
            return val;
        } else {
            unreachable!()
        }
    }

    pub fn majority_element_boyer_moore(nums: Vec<i32>) -> i32 {
        let (mut counter, mut ans) = (0, nums[0]);
        for &n in &nums {
            if counter == 0 {
                ans = n;
            }
            counter += if ans == n { 1 } else { -1 }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 2, 3];
        assert_eq!(3, Solution::majority_element(nums));
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(2, Solution::majority_element(nums));
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 2, 3];
        assert_eq!(3, Solution::majority_element_boyer_moore(nums));
    }

    #[test]
    fn test_4() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(2, Solution::majority_element_boyer_moore(nums));
    }
}
