fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: u32 = input.trim().parse().unwrap();
    let mut d: usize;
    let mut c: u32;
    let mut a: u32;    
    let mut b: u32;
    let mut power: [u32; 32] = [0; 32];
    for i in 0..32 {
        power[i] = 1 << i;
    }
    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        c = input.trim().parse().unwrap();
        // print_bits(c);
        d = 0;
        while power[d] <= c {
            d += 1;
        }
        a = power[d-1];
        b = power[d-1]-1;
        for i in 0..d-1 {
            if c & power[i] == 0 {
                a = a | power[i];
            }
        }
        // print_bits(a);
        // print_bits(b);
        println!("{}", (a as u64)*(b as u64));
    }
}

fn print_bits(num: u32) {
    for i in (0..32).rev() {
        if (num & (1 << i)) == 0 {
            print!("0");
        } else {
            print!("1");
        }
    }
    println!("");
}