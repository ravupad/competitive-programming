// leetcode https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/531/week-4/3307/
// array

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum_frequency_count = HashMap::new();
        let mut sum = 0;
        let mut result = 0;
        sum_frequency_count.insert(0, 1);
        for i in 0..nums.len() {
            sum += nums[i];
            if let Some(count) = sum_frequency_count.get(&(sum-k)) {
                result += count;
            }
            if let Some(count) = sum_frequency_count.insert(sum, 1) {
                sum_frequency_count.insert(sum, count+1);
            }
        }
        result
    }
}
