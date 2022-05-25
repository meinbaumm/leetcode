// https://leetcode.com/problems/find-closest-number-to-zero/
// 2239. Find Closest Number to Zero

fn find_closest_number(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    nums.iter()
        .fold((i32::MIN, i32::MAX), |(val, dist), &x| {
            match dist.cmp(&(x.abs())) {
                Ordering::Greater => (x, x.abs()),
                Ordering::Equal => (x.max(val), dist),
                Ordering::Less => (val, dist),
            }
        })
        .0
}