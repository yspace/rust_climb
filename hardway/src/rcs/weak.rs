use std::rc::{Rc, Weak};

// @see https://stackoverflow.com/questions/60508705/is-there-a-way-to-have-a-struct-contain-a-reference-that-might-no-longer-be-vali
// @see https://www.reddit.com/r/rust/comments/cnjhup/idiomatic_way_to_reference_parent_struct/
struct Whatever {
    thing: Weak<i32>,
}

impl Whatever {
    fn do_it(&self) {
        match self.thing.upgrade() {
            Some(value) => println!("{}", value),
            None => println!("doesn't exist"),
        }
    }
}

fn its_dead_jim() -> Whatever {
    let owner = Rc::new(42);
    let thing = Rc::downgrade(&owner);

    let whatever = Whatever { thing };
    whatever.do_it();

    whatever
}

#[test]
fn main() {

    let whatever = its_dead_jim();
    whatever.do_it();
}
