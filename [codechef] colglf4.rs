use std::cmp::min;

struct State {
    pub n: i32,
    pub e: i32,
    pub h: i32,
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub ca: i32,
    pub cb: i32,
    pub cc: i32,
}

impl State {
    pub fn init(&mut self, n: i32, e: i32, h: i32, ca: i32, cb: i32, cc: i32) {
        self.n = n;
        self.e = e;
        self.h = h;
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.ca = ca;
        self.cb = cb;
        self.cc = cc;
    }

    pub fn buy(&mut self, a: i32, b: i32, c: i32) {
        self.n -= a+b+c;
        self.e -= 2*a+c;
        self.h -= 3*b+c;
        self.a += a;
        self.b += b;
        self.c += c;
    }
}

fn main() {
    let mut string = String::with_capacity(50);
    let mut input: [i32; 6] = [0; 6];
    let mut state = State { n: 0, e: 0, h: 0, a: 0, b: 0, c: 0, ca: 0, cb: 0, cc: 0 };
    let mut min_cost: Option<i64>;
    let (t, mut max_a): (u32, i32);
    read_line(&mut string);
    t = string.trim().parse().unwrap();
    for _ in 0..t {
        read_line(&mut string);
        for (i, num) in string.trim().split(' ').map(|num| num.parse().unwrap()).enumerate() {
            input[i] = num;
        }
        state.init(input[0], input[1], input[2], input[3], input[4],input[5]);
        max_a = min(state.n, state.e/2);
        min_cost = None;
        for a in 0..(max_a+1) {
            state.buy(a, 0, 0);
            if let Some(cost) = solve_for_fixed_a(&mut state) {
                min_cost = min_cost.map(|mc| min(mc, cost)).or(Some(cost));
            }
            state.init(input[0], input[1], input[2], input[3], input[4],input[5]);
        }
        match min_cost {
            Some(cost) => println!("{}", cost),
            None => println!("-1"),
        }
    }
}

fn solve_for_fixed_a(state: &mut State) -> Option<i64> {
    let max_c = min(state.n, min(state.e, state.h));
    state.buy(0, 0, max_c);
    let max_b = min(state.n, state.h/3);
    state.buy(0, max_b, 0);
    if state.cb < state.cc {
        let max_exchange = min(state.c, state.h/2);
        state.buy(0, max_exchange, -1*max_exchange);
    }
    match state.n {
        0 => Some(state.ca as i64*state.a as i64+state.cb as i64*state.b as i64+state.cc as i64*state.c as i64),
        _ => None,
    }
}

fn read_line(string: &mut String) {
    string.clear();
    std::io::stdin().read_line(string).unwrap();
}