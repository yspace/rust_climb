
use std::collections::HashMap ;

pub fn main() {

    fn foo() {
        println!("foo is called") ;
    }

    let mut m = HashMap::new() ;
    m.insert("foo",foo)  ;

    let f = m.get("foo").unwrap() ;
    f() ;

}