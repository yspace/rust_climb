use std::sync::{Arc, Mutex};
use std::thread;

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} has finished eating.", self.name);
    }
}

pub fn main() {}

mod versions {
    use std::thread;

    struct Philosopher {
        name: String,
    }

    impl Philosopher {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }

        fn eat(&self) {
            println!("{} is eating.", self.name);

            thread::sleep_ms(1000);

            println!("{} has finished eating.", self.name);
        }
    }

    pub fn v2() {
        let f1 = Philosopher::new("Judith Butler");
        let f2 = Philosopher::new("Gilles Deleuze");
        let f3 = Philosopher::new("Karl Marx");
        let f4 = Philosopher::new("Emma Goldman");
        let f5 = Philosopher::new("Michel Foucault");

        let philosophers = vec![f1, f2, f3, f4, f5];

        let handles: Vec<_> = philosophers
            .into_iter()
            .map(|f| {
                thread::spawn(move || {
                    f.eat();
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }
    }

    pub fn v1() {
        let f1 = Philosopher::new("Judith Butler");
        let f2 = Philosopher::new("Gilles Deleuze");
        let f3 = Philosopher::new("Karl Marx");
        let f4 = Philosopher::new("Emma Goldman");
        let f5 = Philosopher::new("Michel Foucault");

        let philosophers = vec![f1, f2, f3, f4, f5];

        for f in &philosophers {
            f.eat();
        }
    }
}
