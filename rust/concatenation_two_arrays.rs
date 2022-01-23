// https://leetcode.com/problems/concatenation-of-array/
// Concatenation of Array return [arr, arr]

fn main() {
    let array = vec![1, 2, 3];
    println!("{:?}", get_concatenation(array));

    let another_array = vec![42, 18, 10];
    println!("{:?}", get_concatenation_faster(another_array));
} 

fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let clone_array = nums.clone();
    [nums, clone_array].concat()
}

fn get_concatenation_faster(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        nums.push(nums[i]);
    }

    nums
}