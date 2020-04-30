struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        is s.len() == 0 {
            return true;
        }
        let chars = s.chars().collect::<Vec<_>>();
        let mut is_valid = vec![vec![false; chars.len()]; chars.len()];
        for i in 0..chars.len() {
            is_valid[i][i] = if chars[i] == '*' {
                true
            } else {
                false
            };
        }
        for len in 2..chars.len()+1 {
            for i in 0..chars.len()-len+1 {
                if (chars[i] == '(' || chars[i] == '*')
                    && (chars[i+len-1] == ')' || chars[i+len-1] == '*' )
                    && (len == 2 || is_valid[i+1][i+len-2])
                {
                    is_valid[i][i+len-1] = true;
                    continue;
                }
                for j in i..i+len-1 {                    
                    if is_valid[i][j] && is_valid[j+1][i+len-1] {
                        is_valid[i][i+len-1] = true;
                        break;
                    }
                }
            }
        }
        is_valid[0][chars.len()-1]
    }
}

fn main() {
    let s = "(*()";
    println!("{}", Solution::check_valid_string(s.to_string()));
}
