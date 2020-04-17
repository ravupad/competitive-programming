struct Solution;

impl Solution {
    fn get_digit_square_sum(n: i32) -> i32 {
        let mut digit;
        let mut n = n;
        let mut sum = 0;
        loop {
            if n == 0 {
                return sum;
            }
            digit = n%10;
            sum += digit*digit;
            n /= 10;
        }
    }
    
    pub fn is_happy(n: i32) -> bool {
        let mut happy = Vec::new();
        for _ in 1..1000 {
            happy.push(false);
        }
        let mut n = if n > 1000 { Solution::get_digit_square_sum(n) } else { n };
        loop {
            if happy[n as usize] {
                return false; // we have a cycle
            }
            match n {
                1 => return true,
                _ => {
                    happy[n as usize] = true; // n is visited
                    n = Solution::get_digit_square_sum(n);
                }
            }
        }
    }
}

fn main() {
   println!("{:?}", Solution::is_happy(20));
}
