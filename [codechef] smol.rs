fn main() {
    let mut line = String::new();
    let t: u32;
    let mut nums: [u32; 2] = [0, 0];
    std::io::stdin().read_line(&mut line).unwrap();
    t = line.trim().parse().unwrap();
    for _ in 0..t {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut split = line.trim().split(" ");
        nums[0] = split.next().unwrap().parse().unwrap();
        nums[1] = split.next().unwrap().parse().unwrap();
        if nums[1] == 0 {
            println!("{}", nums[0]);
        } else if nums[1] > nums[0] {
            println!("{}", nums[0]);
        } else {
            println!("{}", nums[0]-nums[1]*(nums[0]/nums[1]));
        }
    }
}
