// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/

use crate::Solution;

impl Solution {
    pub fn move_zeroes_retain(nums: &mut Vec<i32>) {
        let mut z = Vec::new();
        nums.retain(|x| {
            if *x != 0 {
                true
            } else {
                z.push(0);
                false
            }
        });
        let _ = &nums.append(&mut z);
    }

    pub fn move_zeroes_alternative(nums: &Vec<i32>) -> Vec<i32> {
        let zeroes = nums.iter().filter(|&&x| x == 0).cloned();
        let non_zeroes = nums.iter().filter(|&&x| x != 0).cloned();
        non_zeroes.chain(zeroes).collect()
    }

    pub fn move_zeroes_sort(nums: &mut Vec<i32>) {
        nums.sort_by_key(|&x| x == 0)
    }

    pub fn move_zeroes_swap(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
        }
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeroes: Vec<i32> = nums.iter().filter(|&&x| x == 0).cloned().collect();
        let mut non_zeroes: Vec<i32> = nums.iter().filter(|&&x| x != 0).cloned().collect();
        non_zeroes.append(&mut zeroes);
        *nums = non_zeroes;
    }
}

#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let v = vec![0, 1, 0, 3, 12];
        let true_result = vec![1, 3, 12, 0, 0];
        assert_eq!(true_result, Solution::move_zeroes_alternative(&v));
    }

    #[test]
    fn test_2() {
        let mut v = vec![0, 1, 0, 3, 12];
        let true_result = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut v);
        assert_eq!(true_result, v);
    }
}
