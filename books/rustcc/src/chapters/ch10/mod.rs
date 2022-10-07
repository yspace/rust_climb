pub fn run() {
    // pointer_in_rust();
    smart_pointer_box::run();
    interior_mutability::run();
}

fn pointer_in_rust() {
    let x = 10;
    let y = 20;
    let x_p = &x;

    println!("x={}, &x={:p}", x, &x);
    println!("y={}, &y={:p}", y, &y);
    println!("x_p={:p}", x_p);
}

mod smart_pointer_box {
    pub fn run() {
        let x = Box::new(10.5);

        println!("x={}", x);

        // ##
        recursive_type();
        // 
        rc() ;
    }

    fn recursive_type() {
        enum A {
            B(i32),
            // C(i32, A) ,
            C(i32, Box<A>),
        }

        // === main ===
        let x = A::C(1, Box::new(A::C(2, Box::new(A::B(3)))));
        let sz = std::mem::size_of_val(&x);
        println!("size of x = {}", sz);
    }

    fn rc() {
        use std::rc::Rc;
        enum A {
            B(i32),
            C(i32, Rc<A>),
        }

        // ### main
        let x = Rc::new(A::C(1, Rc::new(A::C(2, Rc::new(A::B(3))))));
        println!("count of x = {}", Rc::strong_count(&x));
        let _y = A::C(5, Rc::clone(&x));
        println!("count of x = {}", Rc::strong_count(&x));
        {
            let _z = A::C(10, Rc::clone(&x));
            println!("count of x = {}", Rc::strong_count(&x));
        }
        println!("count of x = {}", Rc::strong_count(&x));
    }
}


mod interior_mutability{
    use std::{rc::Rc, cell::RefCell};

    
    #[derive(Debug)]
    enum A {
   B(i32),
      C(Rc<RefCell<i32>>, Rc<A>),
    }
    pub fn run() {
        let x = Rc::new(RefCell::new(10));
        let y = Rc::new(A::C(Rc::clone(&x), Rc::new(A::B(1))));
         println!("y = {:?}", y);
        *x.borrow_mut() += 100;
        println!("y mutated = {:?}", y);
    }
}