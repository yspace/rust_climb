
pub fn main() {

    // 在rust中，函数是一等公民（可以储存在变量/数据结构中，可以作为参数传入函数，
    // 可以作为返回值），所以rust的函数参数不仅可以是一般的类型，也可以是函数
    say_what("xiaoli", hi) ;
    say_what("xiaoMing", hello) ;

    fn foo() {println!("call foo!"); }
    fn bar() {println!("call bar!"); }
    let kn = KnowledgeNode::new("foo", foo) ;
    println!("kn: {:?}", kn) ;
    
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