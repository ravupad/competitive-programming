struct MinStack {
    pub vec: Vec<i64>,
    pub min: i64,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            vec: Vec::new(),
            min: 0,
        }
    }
    
    fn push(&mut self, x: i32) {
        let x = x as i64;
        if self.vec.len() == 0 {
            self.vec.push(x);
            self.min = x;
        } else if x < self.min {
            self.vec.push(x+(x-self.min));
            self.min = x;
        } else {
            self.vec.push(x);
        }        
    }
    
    fn pop(&mut self) {
        let x = self.vec.pop().unwrap();
        if x < self.min {
            self.min = self.min+(self.min-x);
        }
    }
    
    fn top(&self) -> i32 {
        let x = *(self.vec.last().unwrap());
        if x < self.min {
            self.min as i32
        } else {
            x as i32
        }        
    }
    
    fn get_min(&self) -> i32 {
        self.min as i32
    }
}

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(std::i32::MAX);
    min_stack.push(std::i32::MIN);
    println!("{}", min_stack.get_min());
    min_stack.pop();
    println!("{}", min_stack.get_min());
}
