// 258. Add Digits
// https://leetcode.com/problems/add-digits/
// https://en.wikipedia.org/wiki/Digital_root

use crate::Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 {
            return num;
        }

        let mut digits = num.to_string();
        let mut result = 0;

        loop {
            for c in digits.chars() {
                if let Some(i) = c.to_digit(10) {
                    result += i as i32;
                }
            }

            digits = result.to_string();

            if digits.len() == 1 {
                break;
            }
            result = 0;
        }
        result
    }

    pub fn add_digits_opt(num: i32) -> i32 {
        match num {
            0 => 0,
            _ => 1 + (num - 1) % 9,
        }
    }

    pub fn add_digits_short(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }

    pub fn add_digits_simple(mut num: i32) -> i32 {
        match num {
            0 => return 0,
            _ => {
                num = num % 9;
                if num == 0 {
                    return 9;
                } else {
                    return num;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::add_digits(3));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::add_digits(38));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::add_digits_opt(3));
    }

    #[test]
    fn test_4() {
        assert_eq!(2, Solution::add_digits_opt(38));
    }

    #[test]
    fn test_5() {
        assert_eq!(3, Solution::add_digits_short(3));
    }

    #[test]
    fn test_6() {
        assert_eq!(2, Solution::add_digits_short(38));
    }

    #[test]
    fn test_7() {
        assert_eq!(3, Solution::add_digits_simple(3));
    }

    #[test]
    fn test_8() {
        assert_eq!(2, Solution::add_digits_simple(38));
    }
}
