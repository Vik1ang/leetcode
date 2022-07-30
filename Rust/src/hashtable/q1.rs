use std::collections::HashMap;

pub struct Solution1 {}

impl Solution1 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashtable = HashMap::with_capacity(nums.len());
        for i in 0..Vec::len(&nums) {
            if let Some(key) = hashtable.get(&(target - nums[i])) {
                return vec![*key as i32, i as i32];
            }
            hashtable.insert(nums[i], i);
        }
        vec![]
    }
}
