use std::fs::read_to_string;

pub fn main() {
    println!("learn string");

    // let contents = std::fs::read("runtimes/faust.txt").unwrap() ;
    let contents = std::fs::read_to_string("runtimes/faust.txt").unwrap();
    println!("contents : {}", contents);

    print_me2(&"this is some string parameter") ;
    let msg = String::from("hello string") ;
    // A String type can be magically turned into a &str type using the Deref trait and type coercion.
    //  This will make more sense with an example.
    print_me2(&msg) ;

    deref_coecion::run() ;
    with_struct::run() ; 
}
fn print_me(msg: String) {
    println!("the message is {}", msg);
}

fn print_me2(msg : &str) {
    // 借用
    println!("the message is {}", msg);
}
mod deref_coecion{
    fn print_me(msg: &str) { println!("msg = {}", msg); }
    pub fn run(){
        let string = "hello world";
        print_me(string);
    
        let owned_string = "hello world".to_string(); // or String::from_str("hello world")
        print_me(&owned_string);
    
        let counted_string = std::rc::Rc::new("hello world".to_string());
        print_me(&counted_string);
    
        let atomically_counted_string = std::sync::Arc::new("hello world".to_string());
        print_me(&atomically_counted_string);
    }
}

mod with_struct {
    struct Person<'a>{
        name: &'a str
    }
    impl<'a> Person<'a>{
        fn greet(&self) {
            println!("Hello, my name is {}", self.name);
        }
    }

    pub fn run(){
        /**
         * 是否应该让结构体依赖引用的注意事项
         * - 需要变量的所有权么？
         * - 是否需要在结构体外使用变量
         * - 类型是不是很大  借用不会导致大批量的数据被拷贝
         */

        let person = Person { name: "Herman" };
         person.greet();
    }
    mod v2{
        struct Person {
            // The 'static lifetime is valid for the entire program. You may not need Person or name to live that long.
            name: &'static str,
        }
        
        impl Person {
            fn greet(&self) {
                println!("Hello, my name is {}", self.name);
            }
        }
        
    }
}
