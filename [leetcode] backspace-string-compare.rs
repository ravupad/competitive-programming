struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let a = Solution::resolve_backspace(s.chars().collect::<Vec<_>>());
        let b = Solution::resolve_backspace(t.chars().collect::<Vec<_>>());
        a == b
    }

    pub fn resolve_backspace(mut s: Vec<char>) -> Vec<char> {
        let mut len = 0;
        for i in 0..s.len() {
            if s[i] == '#' {
                len = if len == 0 { 0 } else { len-1 };
            } else {
                s[len] = s[i];
                len += 1;
            }
        }
        s.truncate(len);
        s
    }
}

fn main() {
    println!("{:?}", Solution::backspace_compare("a#de#aa#####".to_string(), "b#df#######".to_string()));
}
