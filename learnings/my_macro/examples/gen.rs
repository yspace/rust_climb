pub mod generated{
    use my_macro::generate ;
    generate!("");
}

use generated::* ;
fn main() {
    println!("gen struct from json schema");
    let s = include_str!("../fixtures/person.json");
    println!("{:#?}",s);

}