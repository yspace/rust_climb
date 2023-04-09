// @see https://stackoverflow.com/questions/37504162/shared-circular-references-in-rust

use std::collections::HashMap;
use std::sync::{Arc, Weak, Mutex};

type Id = u32;

struct ThreadPool {
    inner: Arc<Mutex<ThreadPoolInner>>,
}

struct ThreadPoolInner {
    pool: HashMap<Id, Arc<Mutex<ThreadInner>>>,
    id_count: Id,
}

impl ThreadPool {
    fn new() -> ThreadPool {
        let inner = ThreadPoolInner {
            pool: HashMap::new(),
            id_count: 0,
        };
        ThreadPool { inner: Arc::new(Mutex::new(inner)) }
    }

    fn create(&self) -> Thread {
        let mut inner = self.inner.lock().unwrap();
        let thread = Thread {
            inner: Arc::new(Mutex::new(ThreadInner {
                id: inner.id_count,
                pool: Arc::downgrade(&self.inner),
            })),
        };
        inner.id_count += 1;
        let id = inner.id_count;
        inner.pool.insert(id, thread.inner.clone());
        thread
    }

    fn get(&self, id: Id) -> Option<Thread> {
        let inner = self.inner.lock().unwrap();
        inner.pool.get(&id).map(|t| Thread { inner: t.clone() })
    }

    fn some_mut_method(&self) {
        let _inner = self.inner.lock().unwrap();
        println!("something with pool");
    }
}

struct Thread {
    inner: Arc<Mutex<ThreadInner>>,
}

impl Thread {
    fn get_pool(&self) -> Option<ThreadPool> {
        let inner = self.inner.lock().unwrap();
        // pool is a weak reference, upgrate try to get an Arc from it
        inner.pool.upgrade().map(|inner| ThreadPool { inner: inner })
    }

    fn some_mut_method(&self) {
        if let Some(pool) = self.get_pool() {
            pool.some_mut_method();
            let _t2 = pool.get(2).unwrap();
            println!("something with t2");
        }
    }
}

#[derive(Clone)]
struct ThreadInner {
    id: Id,
    pool: Weak<Mutex<ThreadPoolInner>>,
}

#[test]
fn main() {
    let pool = ThreadPool::new();
    let t1 = pool.create();
    let _t2 = pool.create();
    t1.some_mut_method();
}