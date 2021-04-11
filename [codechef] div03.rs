fn main() {
    let mut line = String::new();
    let t: u32;
    let mut a: [u32; 10] = [0; 10];
    let mut k: u32;
    let mut counter: usize;
    std::io::stdin().read_line(&mut line).unwrap();
    t = line.trim().parse().unwrap();
    for _ in 0..t {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        for (idx, num) in line.trim().split(" ").enumerate() {
            a[idx] = num.parse().unwrap();
        }
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        k = line.trim().parse().unwrap();
        counter = 9;
        while k > 0 {
            if a[counter] > k {
                a[counter] -= k;
                k = 0;
            } else {
                k -= a[counter];
                a[counter] = 0;
                counter -= 1;
            }
        }
        while a[counter] == 0 {
            counter -= 1;
        }
        println!("{}", counter+1);
    }
}
