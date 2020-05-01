// leetcode https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3303/
// dynamic programming dynamic-programming dp

use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;    
        }
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        dp[0][0] = grid[0][0];
        for i in 1..grid.len() {
            dp[i][0] = dp[i-1][0]+grid[i][0];
        }
        for i in 1..grid[0].len() {
            dp[0][i] = dp[0][i-1]+grid[0][i];
        }
        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                dp[i][j] = min(dp[i-1][j], dp[i][j-1])+grid[i][j];
            }
        }
        dp[grid.len()-1][grid[0].len()-1]
    }
}
