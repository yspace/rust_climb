通过结构体创建新类型时 典型地为字段使用String类型 当在函数中同文本交互时通常使用&str .&str 是底层文本的一个视图

### 解引用强制转换

～～～rs
fn print_me(msg: &str) { println!("msg = {}", msg); }

fn main() {
    let string = "hello world";
    print_me(string);

    let owned_string = "hello world".to_string(); // or String::from_str("hello world")
    print_me(&owned_string);

    let counted_string = std::rc::Rc::new("hello world".to_string());
    print_me(&counted_string);

    let atomically_counted_string = std::sync::Arc::new("hello world".to_string());
    print_me(&atomically_counted_string);
}
~~~
You can also use Deref coercion with other types, such as a Vector. After all, a String is just a vector of 8-byte chars

A String type can be magically turned into a &str type using the Deref trait and type coercion


String 内部本质上是一个u8类型的向量
Vec<u8>   可以方便修改
我们对String是拥有所有权的

对于&str 我们只能读取它

结构体成员经常用String 函数参数经常用&str