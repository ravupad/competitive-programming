// leetcode https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3302/
// dfs

struct Solution; 

impl Solution {
    fn dfs(i: usize, j: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
        visited[i][j] = true;
        if i >= 1 && grid[i-1][j] == '1' && !visited[i-1][j] {
            Solution::dfs(i-1, j, grid, visited);
        }
        if i+1 < grid.len() && grid[i+1][j] == '1' && !visited[i+1][j] {
            Solution::dfs(i+1, j, grid, visited);
        }
        if j >= 1 && grid[i][j-1] == '1' && !visited[i][j-1] {
            Solution::dfs(i, j-1, grid, visited);
        }
        if j+1 < grid[0].len() && grid[i][j+1] == '1' && !visited[i][j+1] {
            Solution::dfs(i, j+1, grid, visited);
        }
    }
    
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let mut nums = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if visited[i][j] == false && grid[i][j] == '1' {
                    Solution::dfs(i, j, &grid, &mut visited);
                    nums += 1;
                }
            }
        }
        nums
    }
}

fn main() {
    let a = vec![
        vec!['1','1','1','1','0'],
        vec!['1','1','0','1','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','0','0','0']];
    println!("{}", Solution::num_islands(a));
}

    
