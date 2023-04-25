use std::cell::RefCell;
use std::sync::Mutex;
//do ask rust compiler what types are unwindsafe.
fn implements<T: std::panic::UnwindSafe>() {}

fn main() {
    //可变不共享，共享不可变！
    //包括内部可变性！
    //对于可变且共享的元素，可否证明安全？

    //below all is UnwindSafe.
    implements::<Option<i32>>();
    implements::<&Option<i32>>();
    implements::<&Mutex<i32>>();

    //below all is not UnwindSafe.
    // implements::<&mut i32>();
    // implements::<&RefCell<i32>>();
}

mod _poisoning {
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn main() {
        let lock = Arc::new(Mutex::new(0_u32));
        let lock2 = lock.clone();

        let _ = thread::spawn(move || -> () {
            // This thread will acquire the mutex first, unwrapping the result of
            // `lock` because the lock has not been poisoned.
            let _guard = lock2.lock().unwrap();

            // This panic while holding the lock (`_guard` is in scope) will poison
            // the mutex.
            panic!();
        })
        .join();

        // The lock is poisoned by this point, but the returned result can be
        // pattern matched on to return the underlying guard on both branches.
        let mut guard = match lock.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        println!("{}", *guard);
        *guard += 1;
        println!("{}", *guard);

        assert_eq!(lock.is_poisoned(), true);
        println!("poisoned: {}", lock.is_poisoned());
    }
}

mod _assert_unwind_safe {
    use std::panic::{self, AssertUnwindSafe};

    fn main() {
        let mut variable = 4;
        println!("{}", variable);
        // This code will not compile because the closure captures `&mut variable`
        // which is not considered unwind safe by default.

        // panic::catch_unwind(|| {
        //     variable += 3;
        // });

        // This, however, will compile due to the `AssertUnwindSafe` wrapper
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            variable += 3;
        }));

        println!("{}", variable);
        println!("{:?}", result);
    }
}
