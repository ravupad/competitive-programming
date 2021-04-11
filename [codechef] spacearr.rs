fn main() {
    let mut string = String::with_capacity(2*100_00_0*6);
    read_line(&mut string);
    let t: u32 = string.trim().parse().unwrap();
    let mut n: u32;
    let mut arr: [u32; 2*100_00_0] = [0; 2*100_00_0];
    let mut diff: u32;
    for _ in 0..t {
        read_line(&mut string);
        n = string.trim().parse().unwrap();
        read_line(&mut string);
        for (i, num) in string.trim().split(' ').enumerate() {
            arr[i] = num.parse().unwrap();
        }
        {
            let slice = &mut arr[0..n as usize];
            slice.sort();
        }
        diff = 0;
        for i in 0..n {
            if arr[i as usize] > i+1 {
                diff = 0;
                break;
            }
            diff = diff + i + 1 - arr[i as usize];
        }
        if diff % 2 == 1 {
            println!("First");
        } else {
            println!("Second");
        }
    }
}

fn read_line(string: &mut String) {
    string.clear();
    std::io::stdin().read_line(string).unwrap();
}