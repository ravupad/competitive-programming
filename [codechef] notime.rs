// codechef march21c notime
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let h: u32;
    let x: u32;
    {
        let mut split = line.trim().split(" ");
        split.next();
        h = split.next().unwrap().parse().unwrap();
        x = split.next().unwrap().parse().unwrap();
    }    
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let split = line.trim().split(" ");
    let mut max_t: u32 = 0;
    let mut temp_t: u32;
    for t in split {
        temp_t = t.parse().unwrap();
        if max_t < temp_t {
            max_t = temp_t;
        }
    }
    if h <= x + max_t {
        println!("YES");
    } else {
        println!("NO");
    }
}
