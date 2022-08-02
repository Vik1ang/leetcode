use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub struct Solution1 {}

impl Solution1 {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut window = HashMap::with_capacity(s.len());
        let (mut left, mut right, mut res) = (0, 0, 0);

        let mut s_arr = s.chars().collect::<Vec<_>>();

        while right < s_arr.len() {
            let c = s_arr[right];
            right += 1;
            // 更新窗口内数据
            let count = window.entry(c).or_insert(0);
            *count += 1;

            // 判断左侧窗口是否需要收缩
            while let Some(cnt) = window.get(&c) {
                if *cnt > 1 {
                    let d = s_arr[left];
                    left += 1;
                    // 进行窗口更新
                    let count = window.get_mut(&d).unwrap();
                    *count -= 1;
                } else {
                    break;
                }
            }

            res = max(res as i32, (right - left) as i32);
        }

        res
    }
}

pub struct Solution2 {}

impl Solution2 {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right, mut res) = (0, 0, 0);
        let mut s_arr = s.chars().collect::<Vec<_>>();
        let mut window = HashSet::with_capacity(s.len());

        s_arr.iter().enumerate().for_each(|(index, ch)| {
            while window.contains(ch) {
                window.remove(&s_arr[left as usize]);
                left += 1;
            }
            window.insert(ch);
            res = res.max(right - left + 1);
            right += 1;
        });

        res
    }
}

pub struct Solution3 {}

impl Solution3 {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right, mut res) = (0, 0, 0);
        let mut window = vec![0; 128];

        s.chars().enumerate().for_each(|(index, ch)| {
            left = left.max(window[ch as usize]);
            res = res.max(index as i32 - left + 1);
            window[ch as usize] = index as i32 + 1;
        });

        res
    }
}