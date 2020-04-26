impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_count = 0;
        for i in 0..nums.len() {
            match nums[i] {
                0 => zero_count += 1,
                x => {
                    nums[i-zero_count] = x;
                    if zero_count != 0 {  nums[i] = 0; }
                }
            }        
        }
    }
}
