use std::collections::HashMap;

pub struct Solution1 {}

impl Solution1 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashtable = HashMap::with_capacity(nums.len());
        for (index, &num) in nums.iter().enumerate() {
            if let Some(key) = hashtable.get(&(target - num)) {
                return vec![*key,index as i32]
            }
            hashtable.insert(num, index as i32);
        }
        vec![]
    }
}
