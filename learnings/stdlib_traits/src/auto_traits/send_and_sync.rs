
/*
Almost all types are Send and Sync. 
The only notable Send exception is Rc and the only notable Sync exceptions are Rc, Cell, RefCell.
 If we need a Send version of Rc we can use Arc.
  If we need a Sync version of Cell or RefCell we can Mutex or RwLock. 
  Although if we're using the Mutex or RwLock to just wrap a primitive type it's often better to use the atomic primitive types provided by the standard library such as AtomicBool, AtomicI32, AtomicUsize, and so on.
 */

mod _1{
    use crossbeam::thread;

    #[test]
    fn main() {
        let mut greeting = String::from("Hello");
        let greeting_ref = &greeting;
    
        thread::scope(|scoped_thread| {
            // spawn 3 threads
            for n in 1..=3 {
                // greeting_ref copied into every thread
                scoped_thread.spawn(move |_| {
                    println!("{} {}", greeting_ref, n); // prints "Hello {n}"
                });
            }
    
            // line below could cause UB or data races but compiler rejects it
            // greeting += " world"; // ❌ cannot mutate greeting while immutable refs exist
        });
    
        // can mutate greeting after every thread has joined
        greeting += " world"; // ✅
        println!("{}", greeting); // prints "Hello world"
    }
     
}

mod _2{
    use crossbeam::thread;
#[test]
fn main() {
    let mut greeting = String::from("Hello");
    let greeting_ref = &mut greeting;

    thread::scope(|scoped_thread| {
        // greeting_ref moved into thread
        scoped_thread.spawn(move |_| {
            *greeting_ref += " world";
            println!("{}", greeting_ref); // prints "Hello world"
        });

        // line below could cause UB or data races but compiler rejects it
        // greeting += "!!!"; // ❌ cannot mutate greeting while mutable refs exist
    });

    // can mutate greeting after the thread has joined
    greeting += "!!!"; // ✅
    println!("{}", greeting); // prints "Hello world!!!"
}
}