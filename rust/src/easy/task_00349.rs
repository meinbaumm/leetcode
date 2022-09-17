// 349. Intersection of Two Arrays
// https://leetcode.com/problems/intersection-of-two-arrays/

use crate::Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let set_a: HashSet<_> = nums1.into_iter().collect();
        let set_b: HashSet<_> = nums2.into_iter().collect();
        let c = set_a.intersection(&set_b);

        let mut result = vec![];

        for el in c {
            result.push(*el);
        }
        result
    }
}

#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(vec![2], Solution::intersection(nums1, nums2));
    }

    #[test]
    fn test_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = Solution::intersection(nums1, nums2);

        assert!(result.iter().all(|&x| vec![4, 9].contains(&x)));
    }
}
