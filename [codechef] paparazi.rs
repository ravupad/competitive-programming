use std::cmp::max;

fn read_line(string: &mut String) {
    string.clear();
    std::io::stdin().read_line(string).unwrap();
}

fn main() {
    let mut string = String::new();
    read_line(&mut string);
    let t: u32 = string.trim().parse().unwrap();
    let mut h = [0; 5*1_0000_0];
    let mut c = [(0, 0); 5*1_0000_0];
    for _ in 0..t {
        read_line(&mut string);
        let n: usize = string.trim().parse().unwrap();
        read_line(&mut string);
        for (i, num) in string.trim().split(' ').enumerate() {
            h[i] = num.parse().unwrap();
        }
        println!("{}", solve(&h[0..n], &mut c[0..n]));
    }
}

fn solve(h: &[u32], c: &mut [(u32, u32)]) -> u32 {
    let points = convex_hull(h, c);
    let mut max_dist = 1;
    for i in 1..points {
        max_dist = max(max_dist, c[i].0-c[i-1].0);
    }
    max_dist
}

fn convex_hull(h: &[u32], c: &mut [(u32, u32)]) -> usize {
    c[0] = (0, h[0]);
    c[1] = (1, h[1]);
    let mut points = 2;
    let (mut y1, mut y2, mut x1, mut x2): (i64, i64, i64, i64);
    for i in 2..(h.len()) {
        points += 1;
        c[points-1] = (i as u32, h[i]);
        while points >= 3 {
            y1 = c[points-2].1 as i64 - c[points-3].1 as i64;
            x1 = c[points-2].0 as i64 - c[points-3].0 as i64;
            y2 = c[points-1].1 as i64 - c[points-2].1 as i64;
            x2 = c[points-1].0 as i64 - c[points-2].0 as i64;
            if y1*x2 <= y2*x1 {
                c[points-2] = c[points-1];
                points -= 1;
            } else {
                break;
            }
        }
    }
    points
}

