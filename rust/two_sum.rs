// https://leetcode.com/problems/two-sum/submissions/

use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let result = two_sum(nums, 9);
    println!("{:?}", result);

}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let missing_number = target - nums[i];
        match cache.get_key_value(&missing_number) {
            Some((_k, v)) => return vec![i as i32, *v as i32],
            None => { cache.insert(nums[i], i); }
        }
    }

    vec![]
}
