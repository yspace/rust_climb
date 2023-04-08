use std::cell::{Ref, RefCell};

trait Animal {
    fn speak(&self);
}

struct Cat {
    times_spoken: RefCell<usize>,
}

impl Cat {
    fn report(&self){
        println!("i sporke {} times", self.times_spoken.borrow());
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!  ");

        // 推迟借用检测从编译期到运行期了
        // *self.times_spoken.borrow_mut() += 1;
        let mut b1 =  self.times_spoken.borrow_mut()  ;
        *b1 += 1 ;
        // let b2 =  self.times_spoken.borrow_mut()  ;
    }
}

fn work_with_animal(animal: &dyn Animal) {
    animal.speak();
}

#[test]
fn test_animal() {
    let cat = Cat { times_spoken: RefCell::new( 0 )};

    work_with_animal(&cat);
    work_with_animal(&cat);
    work_with_animal(&cat);

    cat.report();
}
