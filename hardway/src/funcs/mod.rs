
use std::collections::HashMap;

pub fn main() {

    // 在rust中，函数是一等公民（可以储存在变量/数据结构中，可以作为参数传入函数，
    // 可以作为返回值），所以rust的函数参数不仅可以是一般的类型，也可以是函数
    say_what("xiaoli", hi) ;
    say_what("xiaoMing", hello) ;

    // 存储在数据结构中
    fn foo() {println!("call foo!"); }
    fn bar() {println!("call bar!"); }
    let mut kn = KnowledgeNode::new("foo", foo) ;
    println!("kn: {:?}", kn) ;
    kn.func = bar ;
    println!("kn: {:?}", kn) ;
    kn.func = foo ;

    let mut fm = HashMap::new() ;
    fm.insert("foo", kn) ;
    let kn2 = KnowledgeNode::new("bar",bar)
    .set_description(Some(String::from("hi this is desc"))) ;
    fm.insert("bar", kn2) ;
    for  name in fm.keys() {
        println!("\n===== call {} ==========", name) ;
        let node = fm.get(name).unwrap() ;
        println!("name: {} ", node.name) ;
        
        if let Some(desc) = &node.description {
            println!("description:{} ", desc) ;    
        };
        
        let f = &node.func ;
        f() ;

        println!("===== end call {}/=== \n ", name) ;
    }

}



fn hi(name: &str) {
    println!("hi {}", name) ;
}
fn hello(name: &str) {
    println!("hello {}", name) ;
}

fn say_what(name: &str, func: fn(&str)) {
    func(name) ;
}

#[derive(Debug)]
struct KnowledgeNode{
    name: String ,
    description: Option<String> ,
    func: fn() ,
}

impl KnowledgeNode {
    pub fn new(name: &str, func: fn()) -> Self {
        Self{
            name: String::from(name),
            func,
            description: None,
        }
    }

    pub fn set_description(mut self , desc: Option<String>) -> Self{
        self.description = desc ;
        self 
    }
}