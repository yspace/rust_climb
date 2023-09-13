
fn hello(name: &str){
    println!("hello: {}", name);
}

#[test]
fn test_hello(){
    let s = Box::new(String::from("world"));

    hello(&s);
    // hello(Deref::deref(Deref::deref(&s)))
    let ms: &str = &s ;
    // let ms = Deref::deref(Deref::deref(&s)
    // 隐式coercion 协变？
}