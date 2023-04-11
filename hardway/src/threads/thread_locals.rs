// 有个thread_local crate！

mod check_costs_1 {
    // @see https://matklad.github.io/2020/10/03/fast-thread-locals-in-rust.html
    use std::{cell::Cell, time::Instant};

    thread_local! {
      static COUNTER: Cell<u32> = Cell::new(0);
    }
    const STEPS: u32 = 1_000_000_000;
    fn sum_rust() -> u32 {
        for step in 0..STEPS {
            COUNTER.with(|it| {
                let inc = step.wrapping_mul(step) ^ step;
                it.set(it.get().wrapping_add(inc))
            })
        }
        COUNTER.with(|it| it.get())
    }
    #[test]
    fn main() {
        let t = Instant::now();
        let r = sum_rust();
        eprintln!("Rust:   {} {}ms", r, t.elapsed().as_millis());
    }
}

mod in_my_struct {
    use std::cell::RefCell;

    struct Foo;
    impl Foo {
        thread_local! {
            // Could add pub to make it public to whatever Foo already is public to.
            static FOO: RefCell<usize> = RefCell::new(0);
        }
    }

    #[test]
    fn run() {
        Foo::FOO.with(|x| println!("{:?}", x));
    }
}

mod storing_reference {
    use std::cell::RefCell;
    use std::thread::LocalKey;

    thread_local! {
        // Note lack of pub
        static FOO: RefCell<usize> = RefCell::new(0);
    }
    struct Bar {
        // Visibility here changes what can see `foo`.
        foo: &'static LocalKey<RefCell<usize>>,
        // Rest of your data.
    }
    impl Bar {
        fn constructor() -> Self {
            Self {
                foo: &FOO,
                // Rest of your data.
            }
        }
    }
}

mod mutexes {
    use std::sync::Mutex;

    #[test]
    fn test_mutexes() {
        let value: Mutex<u32> = Mutex::new(0);

        // acquire the mutex into a guard object
        let mut guard = value.lock().unwrap();

        // this "derefs" the guard into &mut u32
        *guard += 42;
    }
}

mod _thread_local {
    use std::cell::RefCell;

    //  laziness — thread locals are initialized on the first access.
    thread_local! {
        static VALUE: RefCell<u32>  = RefCell::new(0);
    }
    #[test]
    fn it_works() {
        // borrow the cell and write into it.
        // *value.borrow_mut() += 42;

        // borrow the cell and write into it.
        VALUE.with(|value| {
            *value.borrow_mut() += 42;
        });
    }
}
