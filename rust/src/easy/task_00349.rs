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

    pub fn intersection_2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counter = [false; 1001];
        nums1.iter().for_each(|&x| counter[x as usize] = true);
        nums2
            .iter()
            .filter(|&&x| match counter[x as usize] {
                true => {
                    counter[x as usize] = false;
                    true
                }
                false => false,
            })
            .cloned()
            .collect()
    }

    pub fn intersection_3(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1.retain(|&x| nums2.contains(&x));
        nums1.sort();
        nums1.dedup();
        nums1
    }

    // https://leetcode.com/problems/intersection-of-two-arrays/discuss/1706150/Rust-Linear-Scan
    pub fn intersection_4(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len1: usize = nums1.len();
        let len2: usize = nums2.len();
        if len1 > len2 {
            return Self::intersection(nums2, nums1);
        }
        const RANGE: u16 = 1e3 as u16 + 7;
        let mut freqs: Vec<u16> = vec![0; RANGE as usize];
        for num in nums1 {
            freqs[num as usize] += 1;
        }
        let mut ans: Vec<i32> = Vec::new();
        for num in nums2 {
            if freqs[num as usize] > 0 {
                ans.push(num);
                freqs[num as usize] = 0;
            }
        }
        ans
    }

    pub fn intersection_5(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let set1: HashSet<_> = nums1.into_iter().collect();
        let set2: HashSet<_> = nums2.into_iter().collect();

        set1.intersection(&set2).map(|n| *n).collect()
    }

    pub fn intersection_two_pointers(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut l: usize = 0;
        let mut r: usize = 0;
        let (n1, n2) = (nums1.len(), nums2.len());

        while l < n1 && r < n2 {
            if nums1[l] == nums2[r] {
                ans.push(nums1[l]);
                while l < n1 - 1 && nums1[l] == nums1[l + 1] {
                    l += 1;
                }
                while r < n2 - 1 && nums2[r] == nums2[r + 1] {
                    r += 1;
                }
                r += 1;
                l += 1;
            } else if nums1[l] < nums2[r] {
                l += 1;
            } else {
                r += 1
            }
        }
        ans
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

    #[test]
    fn test_3() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(vec![2], Solution::intersection_2(nums1, nums2));
    }

    #[test]
    fn test_4() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = Solution::intersection_2(nums1, nums2);

        assert!(result.iter().all(|&x| vec![4, 9].contains(&x)));
    }

    #[test]
    fn test_5() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(vec![2], Solution::intersection_3(nums1, nums2));
    }

    #[test]
    fn test_6() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = Solution::intersection_3(nums1, nums2);

        assert!(result.iter().all(|&x| vec![4, 9].contains(&x)));
    }

    #[test]
    fn test_7() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(vec![2], Solution::intersection_4(nums1, nums2));
    }

    #[test]
    fn test_8() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = Solution::intersection_4(nums1, nums2);

        assert!(result.iter().all(|&x| vec![4, 9].contains(&x)));
    }

    #[test]
    fn test_9() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(vec![2], Solution::intersection_5(nums1, nums2));
    }

    #[test]
    fn test_10() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = Solution::intersection_5(nums1, nums2);

        assert!(result.iter().all(|&x| vec![4, 9].contains(&x)));
    }

    #[test]
    fn test_11() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(vec![2], Solution::intersection_two_pointers(nums1, nums2));
    }

    #[test]
    fn test_12() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = Solution::intersection_two_pointers(nums1, nums2);

        assert!(result.iter().all(|&x| vec![4, 9].contains(&x)));
    }
}
