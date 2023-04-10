pub mod ref_cycles;
pub mod ref_cells;

// https://rauljordan.com/rust-concepts-i-wish-i-learned-earlier/
pub fn run() {
    let mut foo = Foo { x: 10 };
    println!("{}", foo.total()); // WORKS.
    foo.increase(); // ERROR: Foo not declared as mut
}

pub struct Foo {
    x: u64,
}

impl Foo {
    /// Any type that borrows an instance of Foo can
    /// call this method, as it only requires a reference to Foo.
    pub fn total(&self) -> u64 {
        self.x
    }
    /// Only exclusive references to instances of Foo
    /// can call this method, as it requires Foo to be mutable.
    pub fn increase(&mut self) {
        self.x += 1;
    }
}

mod bidirectional_references {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
}

mod derefs {
    use std::ops::{Deref, DerefMut};

    struct Example<T> {
        value: T,
    }

    impl<T> Deref for Example<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<T> DerefMut for Example<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }

    #[test]
    fn test_deref() {
        let mut x = Example { value: 'a' };
        *x = 'b';
        assert_eq!('b', x.value);

        // ===================
        struct Foo {
            value: u64,
        }
        let mut foo = Box::new(Foo { value: 10 });
        
        // Box implements DerefMut, so this will work fine!
        *foo = Foo { value: 20 };
        // Dot methods will work on foo because Box implements Deref.
        // We do not have to worry about the implementation
        // detail that Foo is boxed.
        assert_eq!(20, foo.value);
    }
}
