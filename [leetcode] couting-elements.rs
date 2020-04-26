struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let mut count = HashSet::<i32>::new();
        let mut result = 0;
        for i in 0..arr.len() {
            count.insert(arr[i]);
        }
        for i in 0..arr.len() {
            if count.contains(&(arr[i]+1)) {
                result += 1;
            }
        }
        result
    }
}

fn main() { 
    println!("{:?}", Solution::count_elements(vec![1,2,1,2,3]));
}
