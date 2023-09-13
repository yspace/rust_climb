// #![feature(cell_filter_map)]

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

pub struct MyMap<T> {
    map: Rc<RefCell<HashMap<String, T>>>,
}

impl<T> MyMap<T> {
    pub fn get(&self, key: &str) -> Option<Ref<T>> {
        Ref::filter_map(self.map.borrow(), |map| map.get(key)).ok()
    }

    // context manager:
    pub fn with_value<F, O>(&self, key: &str, f: F) -> O
    where
        F: FnOnce(Option<&T>) -> O,
    {
        f(self.map.borrow().get(key))
    }
}
#[test]
fn main() {
    let map: MyMap<u32> = MyMap {
        map: Rc::new(RefCell::new(HashMap::from([
            ("meaning".to_string(), 42),
            ("nice".to_string(), 69),
        ]))),
    };

    println!("{:?}", map.get("meaning"));


    map.with_value("meaning", |value| {
        println!("{:?}", value);
    });
}