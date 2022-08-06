// 2264. Largest 3-Same-Digit Number in String
// https://leetcode.com/problems/largest-3-same-digit-number-in-string/

use crate::Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.chars()
            .zip(num.chars().skip(1))
            .zip(num.chars().skip(2))
            .filter_map(|((a, b), c)| {
                if a == b && b == c {
                    Some(format!("{}{}{}", a, b, c))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_default()
    }

    pub fn largest_good_integer_buffer(num: String) -> String {
        let mut max_good_int = String::new();
        let mut buffer = Vec::with_capacity(3);
    
        for ch in num.chars() {
            if buffer.is_empty() {
                buffer.push(ch);
                continue;
            }
    
            if *buffer.last().unwrap() != ch {
                buffer.clear();
            }
    
            buffer.push(ch);
    
            if buffer.len() == 3 {
                let value = buffer.iter().collect::<String>();
    
                if max_good_int.is_empty() || value.parse().unwrap_or(0) > max_good_int.parse().unwrap() {
                    max_good_int = value;
                }
            }
        }
    
        max_good_int
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "777", 
            Solution::largest_good_integer("6777133339".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "000", 
            Solution::largest_good_integer("2300019".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "", 
            Solution::largest_good_integer("42352338".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "777", 
            Solution::largest_good_integer_buffer("6777133339".to_string())
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "000", 
            Solution::largest_good_integer_buffer("2300019".to_string())
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            "", 
            Solution::largest_good_integer_buffer("42352338".to_string())
        );
    }
}
