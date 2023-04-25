macro_rules! bar {
    () => {};
}

pub(crate) use bar; // <-- the trick

mod node {
    #![macro_use]

    macro_rules! test {
        () => {
            println!("Nuts");
        };
    }

    macro_rules! best {
        () => {
            println!("Run");
        };
    }

    pub fn fun_times() {
        println!("Is it really?");
    }
}
mod toad {
    use super::node; //Notice this is 'use' not 'mod'

    pub fn a_thing() {
          test!();

          node::fun_times();
    }
}

fn main() {
    test!();
    best!();
    toad::a_thing();
}
