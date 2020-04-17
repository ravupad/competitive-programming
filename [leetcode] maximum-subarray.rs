use std::cmp::max;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = Vec::with_capacity(nums.len());
        let mut sum = 0;
        for i in 0..nums.len() { 
            sum = sum+nums[i];
            prefix_sum.push(sum);
        }
        let mut min_prefix_sum = Vec::with_capacity(nums.len());
        min_prefix_sum.push(min(0, prefix_sum[0]));
        for i in 1..nums.len() {
            min_prefix_sum.push(min(min_prefix_sum[i-1], prefix_sum[i]));
        }
        let mut result = nums[0];
        for i in 1..nums.len() {
            result = max(result, prefix_sum[i] - min_prefix_sum[i-1]);
        }
        result
    }
}

fn main() {
   println!("{:?}", Solution::max_sub_array(vec![1,2,3]));
}
