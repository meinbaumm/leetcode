// 344. Reverse String
// https://leetcode.com/problems/reverse-string/
use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse()
    }
}

#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut v);
        let result: Vec<char> = vec!['o', 'l', 'l', 'e', 'h'];
        assert_eq!(result, v);
    }
}
