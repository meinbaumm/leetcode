// 2248. Intersection of Multiple Arrays
// https://leetcode.com/problems/intersection-of-multiple-arrays/

use crate::Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut set = std::collections::HashSet::new();
        let mut vec = Vec::new();
        
        for num in &nums {
            for n in &num {
                set.insert(n);
            }
        }

        for s in &set {
            vec.push(s);
        }
        vec

    }       
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::intersection(vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]]),
            [3, 4]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            []
        )
    }
}
