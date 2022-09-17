// 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/

use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        map.into_values().all(|v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        assert_eq!(true, Solution::is_anagram(s, t));
    }

    #[test]
    fn test_2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        assert_eq!(false, Solution::is_anagram(s, t));
    }

    #[test]
    fn test_3() {
        let s = "a".to_string();
        let t = "ab".to_string();

        assert_eq!(false, Solution::is_anagram(s, t));
    }

    #[test]
    fn test_4() {
        let s = "aacc".to_string();
        let t = "ccac".to_string();

        assert_eq!(false, Solution::is_anagram(s, t));
    }
}
