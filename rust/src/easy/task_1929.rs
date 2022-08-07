// 1929. Concatenation of Array
// https://leetcode.com/problems/concatenation-of-array/

use crate::Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let clone_array = nums.clone();
        [nums, clone_array].concat()
    }

    pub fn get_concatenation_faster(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            nums.push(nums[i]);
        }

        nums
    }

    pub fn get_concatenation_iter(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().chain(nums.iter()).cloned().collect()
    }

    pub fn get_concatenation_cycle(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().cycle().take(nums.len() * 2).cloned().collect()
    }

    pub fn get_concatenation_vec(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len() * 2];
        nums.iter().enumerate().for_each(|(i, &x)| {
            res[i] = x;
            res[i + nums.len()] = x;
        });
        res
    }

    pub fn get_concatenation_append(mut nums: Vec<i32>) -> Vec<i32> {
        nums.append(&mut nums.clone());
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::get_concatenation_faster(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::get_concatenation_iter(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::get_concatenation_cycle(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::get_concatenation_vec(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::get_concatenation_append(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        );
    }
}
