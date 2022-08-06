// 2114. Maximum Number of Words Found in Sentences
// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

use crate::Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut maximum_words: i32 = 0;
        for s in sentences.iter() {
            let slice: Vec<&str> = s[..s.len()].split(" ").collect();
            match slice.len() > maximum_words.try_into().unwrap() {
                true => maximum_words = slice.len() as i32,
                false => continue,
            };
        }
        maximum_words 
    }

    pub fn another_most_words_found(sentences: Vec<String>) -> i32 {
        sentences.iter().fold(0, |acc, s| {
            acc.max(s.as_bytes().iter().filter(|&&b| b == b' ').count() + 1)
        }) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::most_words_found(
            vec![
                "please wait".to_string(), 
                "continue to fight".to_string(), 
                "continue to win".to_string(),
                ]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::most_words_found(
            vec![
                "alice and bob love leetcode".to_string(), 
                "i think so too".to_string(), 
                "this is great thanks very much".to_string(),
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::another_most_words_found(
            vec![
                "please wait".to_string(), 
                "continue to fight".to_string(), 
                "continue to win".to_string(),
                ]
            )
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(6, Solution::another_most_words_found(
            vec![
                "alice and bob love leetcode".to_string(), 
                "i think so too".to_string(), 
                "this is great thanks very much".to_string(),
                ]
            )
        );
    }
}
