
use std::collections::HashMap ;

pub fn main() {

    fn foo() {
        println!("foo is called") ;
    }

    let mut m = HashMap::new() ;
    m.insert("foo",foo)  ;

    let f = m.get("foo").unwrap() ;
    f() ;

    fn bar() {
        println!("bar is called!") ;
    }
    //
    // m.insert("bar", bar) ; // not allowed by compiler !

    // type MainFn = Box<dyn Fn()->()> ;
    // let m2  = HashMap::new() ; // need specify the generic type params ;
    // m2.insert("f1", Box::new(bar)) ;
    // m2.insert("f1", Box::new(foo)) ;

    /// This is a good point to remember that Fn()->() is a trait,
    ///  not a type. They have a type that implements that trait, 
    /// but they have different types.
    /// the same as usual for storing hetrogeneous types: store Box<dyn Fn()->()>, 
    /// or make an enum.
    // type VoidFnPtr = Box<dyn Fn()->()>;  // this is OK
    type VoidFnPtr = Box< Fn()->()>;
    let mut ans = HashMap::<String, VoidFnPtr>::new();
    ans.insert("foo".to_owned(), Box::new(foo));
    ans.insert("bar".to_owned(), Box::new(bar));

    for fn_key in ans.keys() {
        println!("fn key in hashmap : {}", fn_key) ;

        let f = ans.get(fn_key).unwrap() ;
        f() ;
    }


}