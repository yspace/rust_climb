// @see https://github.com/kumaarpranv/rust_designpatterns/blob/master/src/creational/singleton.rs
use once_cell::sync::Lazy;
use rand::Rng;
use std::sync::{Arc, Mutex};

pub struct Singleton {
    data: u32,
}

impl Singleton {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Singleton {
            data: rng.gen_range(0..10) as u32,
        }
    }

    pub fn instance() -> Arc<Mutex<Self>> {
        // Use once_cell's Lazy type to initialize the instance only once
        static INSTANCE: Lazy<Arc<Mutex<Singleton>>> =
            Lazy::new(|| Arc::new(Mutex::new(Singleton::new())));
        INSTANCE.clone()
    }

    pub fn get_data(&self) -> u32 {
        self.data.clone()
    }
}



#[test]
fn singleton_test() {
    let singleton = Singleton::instance();
    let data = singleton.lock().unwrap().get_data();
    let singleton1 = Singleton::instance();
    let data1 = singleton1.lock().unwrap().get_data();
    assert_eq!(data, data1);
}