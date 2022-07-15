
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn main() {
    // Assuming a is an i32, it takes 4 bytes of memory
    let a = 42 ;
    // Variables c and b are references. 
    // References are 4 bytes wide on 32-bit CPUs and 8 bytes wide on 64-bit CPUs.
    let b = &B ;
    let c = &C ;

    // {:p} 语法让rust 格式化变量为指针 并打印其指向的物理地址
    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}