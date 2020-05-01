// leetcode https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3304/
// binary search; binary-search; array
// rotated sorted array

struct Solution;

impl Solution {
    fn get_min_index(nums: &Vec<i32>) -> usize {
        let mut start = 0;
        let mut end = nums.len()-1;
        let mut mid;
        while start != end {
            mid = (start+end)/2;
            if nums[mid] < nums[end] {
                end = mid;
            } else {
                start = mid+1;
            }            
        }
        start
    }
    
    fn find(nums: &Vec<i32>, start: usize, end: usize, target: i32) -> isize {
        let (mut start, mut end) = (start, end);
        let mut mid;
        while start != end {
            mid = (start+end)/2;
            if nums[mid] >= target {
                end = mid;
            } else {
                start = mid+1;
            }                
        }
        if nums[start] == target {
            start as isize
        } else {
            -1
        }
    }
    
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let min_idx = Self::get_min_index(&nums);
        let solution;
        if min_idx == 0 {
            Self::find(&nums, 0, nums.len()-1, target) as i32
        } else {
            solution = Self::find(&nums, min_idx, nums.len()-1, target);
            if solution != -1 {
                solution as i32
            } else {
                Self::find(&nums, 0, min_idx-1, target) as i32
            }
        }
    }
}

fn main() {
    let vec = vec![4,5,6,1,2,3];
    println!("{}", Solution::search(vec, 10));
}
