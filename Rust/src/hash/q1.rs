use std::collections::HashMap;

trait Q1 {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32>;
}

struct Solution1;
impl Q1 for Solution1 {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            if let Some(key) = hash.get(&(target - nums[i])) {
                if *key != i {
                    return vec![*key as i32, i as i32];
                }
            }
            hash.insert(nums[i], i);
        }
        panic!("not found")
    }
}

pub fn solution() {
    println!("q1")
}
