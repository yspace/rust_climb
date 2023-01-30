use std::{thread, time, sync::{Arc, Mutex}};


pub fn run(){
    
    let h = Arc::new(Thing{});

    for i in 0..10{
        let h = h.clone();

        thread::spawn(move ||{
            h.foo();
        });
    }

    demo2() ;
    
    thread::sleep(time::Duration::from_secs(2));
}

fn demo2(){
    let h = Arc::new(Mutex::new(  Thing{}));

    for i in 0..10{
        let h = h.clone();

        thread::spawn(move ||{
            let mut l = h.lock().unwrap();
            l.foo();
            l.foo2();
        });
    }
    thread::sleep(time::Duration::from_secs(2));
}

struct Thing{

}

impl Thing {
    fn foo(&self) {
        let tid = thread::current().id();
        println!("{:?}", tid);
    }

    fn foo2(&mut self) {
        println!("this is a mutable thing ,you should call this method from a mutable ref");
    }
}