// 35. Search Insert Position
// https://leetcode.com/problems/search-insert-position/

use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let (mut low, mut high) = (0, nums.len());
        while low < high {
            let mid = (high + low) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        low as i32
    }

    pub fn search_insert_binary_search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }

    pub fn search_insert_take_while(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter().take_while(|x| **x < target).count() as i32
    }

    pub fn search_insert_contains(mut nums: Vec<i32>, target: i32) -> i32 {
        if !nums.contains(&target) {
            nums.push(target);
            nums.sort();
        }

        if let Some(x) = nums.iter().position(|&num| num == target) {
            x as i32
        } else {
            unreachable!()
        }
    }

    pub fn search_insert_position(nums: Vec<i32>, target: i32) -> i32 {
        if nums.last().unwrap() < &target {
            return nums.len() as i32;
        }

        if let Some(i) = nums.iter().position(|v| *v == target || *v > target) {
            return i as i32;
        }

        -1i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        assert_eq!(2, Solution::search_insert(nums, target));
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;

        assert_eq!(1, Solution::search_insert(nums, target));
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;

        assert_eq!(4, Solution::search_insert(nums, target));
    }
}
