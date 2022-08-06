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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_concatenation(vec![1, 2, 3]), vec![1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_concatenation(vec![42]), vec![42, 42]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_concatenation_faster(vec![1, 2, 3]), vec![1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::get_concatenation_faster(vec![42]), vec![42, 42]);
    }
}
