pub fn main() {
    let a: i64 = 42;
    //
    // Under the hood, references (&T and &mut T) are implemented as raw pointers
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {} ({:p} ... 0x{:x})", a, a_ptr, a_addr + 7);
}
