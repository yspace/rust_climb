use std::mem::size_of;

// &[u8; 10] reads as “a reference to an array of 10 bytes.” The array is located in static
// memory, and the reference itself (a pointer of width usize bytes) is placed on the stack.
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn main() {
    //  usize is the memory address size for the CPU the code is compiled for. That CPU is called the compile target
    // usize 是代码为特定CPU编译的内存地址大小 ，此CPU称为编译目标
    let a: usize = 42;

    let b: &[u8; 10] = &B;
    // The Box<[u8]> type is a boxed byte slice.
    //  When we place values inside a box,
    // ownership of the value moves to the owner of the box.
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} bytes", size_of::<usize>());
    println!("  value       {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("  location:  {:p}", &b);
    println!("  size:      {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to: {:p}", b);
    println!();

    println!("c (a \"box\" for C):");
    println!("  location:  {:p}", &c);
    println!("  size:      {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to: {:p}", c);
    println!();
    println!("B (an array of 10 bytes):");
    println!(" location: {:p}", &B);
    println!(" size: {:?} bytes", size_of::<[u8; 10]>());
    println!(" value: {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!(" location: {:p}", &C);
    println!(" size: {:?} bytes", size_of::<[u8; 11]>());
    println!(" value: {:?}", C);
}
