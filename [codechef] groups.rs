// codechef march21c groups
fn main() {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let t: u32 = string.trim().parse().unwrap();
    let mut count;
    let mut prev: char;
    for _ in 0..t {
        string.clear();
        std::io::stdin().read_line(&mut string).unwrap();
        prev = '0';
        count = 0;
        for char in string.trim().chars() {
            if char == '1' && prev == '0' {
                count += 1;
            }
            prev = char;
        }
        println!("{}", count);
    }
}