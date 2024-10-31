// https://leetcode.com/problems/two-sum/description/

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap = HashMap::new();
    for (index, num) in nums.into_iter().enumerate() {
        if let Some(idx) = hashmap.get(&(target - num)) {
            return vec![*idx, index as i32];
        }
        hashmap.insert(num, index as i32);
    }
    unreachable!()
}
