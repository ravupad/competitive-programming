fn main() {
    let mut line = String::new();
    let t: u32;
    let mut alphabet_count: [u32; 26] = [0; 26];
    let mut one_count: u32;
    let mut even_count: u32;
    std::io::stdin().read_line(&mut line).unwrap();
    t = line.trim().parse().unwrap();
    for _ in 0..t {
        for i in 0..26 {
            alphabet_count[i] = 0;
        }
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        for char in line.trim().chars() {
            alphabet_count[(char as u32) as usize - 97] += 1;
        }
        one_count = 0;
        even_count = 0;
        for i in 0..26 {
            if alphabet_count[i] == 1 {
                one_count += 1;
            } else if alphabet_count[i] > 0  && alphabet_count[i]%2 == 0 {
                even_count += alphabet_count[i]/2;
            } else if alphabet_count[i] > 0  && alphabet_count[i]%2 != 0 {
                even_count += (alphabet_count[i]-3)/2;
            }
        }
        if one_count > even_count {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}