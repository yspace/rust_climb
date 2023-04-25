use std::panic;

fn main() {
    // This is designed to be used in conjunction with catch_unwind to, for example, carry a panic across a layer of C code.
    // 主要用于FFI, 根据C代码中传出来的Err，在Rust代码中throw a panic.

    let result = panic::catch_unwind(|| {
        panic!("oh no!, panic occured!");
    });

    println!("I am ok 1st",);

    if let Err(err) = result {
        println!("I am ok 2nd",);
        panic::resume_unwind(err);
        //println!("unreachable here", );
    }

    println!("unreachable here",);
}
