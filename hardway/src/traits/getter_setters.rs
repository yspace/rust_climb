trait Test {
    fn get_a(&self) -> i32;
    fn get_b(&self) -> i32;
    fn output(&self) -> i32 {
        self.get_a() + self.get_b()
    }
}

struct TT {
    a: i32,
    b: i32,
}

struct TTT {
    a: i32,
    b: i32,
}

macro_rules! getters {
    {} => {
        fn get_a(&self) -> i32 {
            self.a
        }
        fn get_b(&self) -> i32 {
            self.b
        }
    }
}

impl Test for TT {
    getters!();
}

impl Test for TTT {
    getters!();
    
    fn output(&self) -> i32 {
        self.get_a() - self.get_b()
    }
}

#[test]
fn main() {
    let a = TT {a:1, b:2};
    let b = TTT {a:1, b:2};
    println!("{}", a.output());
    println!("{}", b.output());
}