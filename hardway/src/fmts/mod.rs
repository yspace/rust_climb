#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}


pub fn main() {
    println!("hello fmt package");
    let some_var = 2 ;

    println!("{}", &some_var);
    println!("{:?}", some_var);
    println!("{:x?}", some_var);
    println!("{:X?}", some_var);
    println!("{:o}", some_var);

    println!("{:#X}", &some_var);
    println!("{:#X}", &some_var as *const i32 as usize);

    let p = Person {
        name: "John",
        age: 30,
    };

    println!("{:?}", p);
    println!("{:#?}", p);

}