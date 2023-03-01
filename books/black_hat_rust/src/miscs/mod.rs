mod smart_pointer {
    // The solution for long-lived, shared (or not), mutable (or not) references is to use smart pointers.

    use std::rc::Rc;
    fn main() {
        let pointer = Rc::new(1);
        {
            let second_pointer = pointer.clone(); // or Rc::clone(&pointer)
            println!("{}", *second_pointer);
        }
        println!("{}", *pointer);
    }
    #[test]
    fn test_main() {
        main();
    }
}

mod interior_mutability {
    use std::cell::{RefCell, RefMut};
    use std::rc::Rc;
    fn main() {
        let shared_string = Rc::new(RefCell::new("Hello".to_string()));
        {
            let mut hello_world: RefMut<String> = shared_string.borrow_mut();
            hello_world.push_str(" World");
        }
        println!("{}", shared_string.take());
    }

    #[test]
    fn test_main() {
        main();
    }
}
// 上面的不可以跨线程 也不能在async异步编程中使用 。
// 需要使用Arc 它实现了Send 和Sync trait。 可安全的跨线程共享.
mod arc_shared_ref {
    use std::sync::{Arc, Mutex};
    use std::{thread, time};
    fn main() {
        let pointer = Arc::new(5);
        let second_pointer = pointer.clone(); // or Arc::clone(&pointer)
        thread::spawn(move || {
            println!("{}", *second_pointer); // 5
        });
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", *pointer); // 5
    }
}

mod arc_mutable_share {
    use std::sync::{Arc, Mutex};
    use std::{thread, time};
    fn main() {
        let pointer = Arc::new(Mutex::new(5));
        let second_pointer = pointer.clone(); // or Arc::clone(&pointer)
        thread::spawn(move || {
            let mut mutable_pointer = second_pointer.lock().unwrap();
            *mutable_pointer = 1;
        });
        thread::sleep(time::Duration::from_secs(1));
        let one = pointer.lock().unwrap();
        println!("{}", one); // 1
    }
}
