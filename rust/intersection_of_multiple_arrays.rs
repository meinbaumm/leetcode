// https://leetcode.com/problems/intersection-of-multiple-arrays/
// 2248. Intersection of Multiple Arrays

fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut dict = std::collections::HashMap::new();
    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            *dict.entry(nums[i][j]).or_insert(0) += 1;
        }
    }
    let mut res = vec![];
    for (k, v) in dict {
        if v == nums.len() {
            res.push(k);
        }
    }
    res.sort();
    res
}


fn functional_intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let l = nums.len();
    let mut counter = [0; 1001];
    nums.iter()
        .flat_map(|v| v.iter())
        .for_each(|v| counter[*v as usize] += 1);

    counter
        .iter()
        .zip(0..)
        .filter_map(|(&num, idx)| if num == l { Some(idx) } else { None })
        .collect()
}
