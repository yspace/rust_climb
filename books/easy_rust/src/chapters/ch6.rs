pub fn run() {
    println!("ch6 comments");
}

mod comments {
    pub fn main() {
        // Rust programs start with fn main()
        // You put the code inside a block. It starts with { and ends with }
        let some_number = 100; // We can write as much as we want here and the compiler won't look at it
    }
}
mod comments2{
    pub fn main() {
        let some_nuber /*: i16 */ = 100 ;
    }

    fn main2(){
        let some_number = 100; /* Let me tell you
        a little about this number.
        It's 100, which is my favourite number.
        It's called some_number but actually I think that... */
    
        let some_number = 100; // Let me tell you
        // a little about this number.
        // It's 100, which is my favourite number.
        // It's called some_number but actually I think that...
    }

}
