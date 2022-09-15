// 231. Power of Two
// https://leetcode.com/problems/power-of-two/

use crate::Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let number = n as f32;
        let log = number.log2() as u32;
        2i32.pow(log) == n
    }

    pub fn is_power_of_two_bit(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_power_of_two(16));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::is_power_of_two(3));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, Solution::is_power_of_two(8192));
    }

    #[test]
    fn test_4() {
        assert_eq!(true, Solution::is_power_of_two_bit(16));
    }

    #[test]
    fn test_5() {
        assert_eq!(false, Solution::is_power_of_two_bit(3));
    }

    #[test]
    fn test_6() {
        assert_eq!(true, Solution::is_power_of_two_bit(8192));
    }
}
