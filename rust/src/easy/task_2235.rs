// 2235. Add Two Integers
// https://leetcode.com/problems/add-two-integers/

use crate::Solution;

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::sum(6, 6));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::sum(-6, 6));
    }

    #[test]
    fn test_3() {
        assert_eq!(-12, Solution::sum(-6, -6));
    }
}
