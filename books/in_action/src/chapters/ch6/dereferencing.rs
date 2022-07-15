pub fn main() {
    // Creating a pointer of arbitrary types from any integer is perfectly legal.
    let ptr = 42 as *const Vec<String>;

    // Dereferencing that pointer must occur within an unsafe block
    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}
