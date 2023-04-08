
trait Trait {
    fn method(&self) {
        println!("default impl");
    }
}

struct SomeType;
struct OtherType;

// use default impl for Trait::method
impl Trait for SomeType {}

impl Trait for OtherType {
    // use our own impl for Trait::method
    fn method(&self) {
        println!("OtherType impl");
    }
}

fn main() {
    SomeType.method(); // prints "default impl"
    OtherType.method(); // prints "OtherType impl"
}

//  可以用来实现模版方法设计模式哦！
trait Greet {
    // 你为我实现这个方法 <required-method>
    fn greet(&self, name: &str) -> String;
    // 我就给你提供这个方法 <surpported-method> ；当然你也可以选择覆盖实现
    fn greet_loudly(&self, name: &str) -> String {
        self.greet(name) + "!"
    }
}

struct Hello;
struct Hola;

impl Greet for Hello {
    fn greet(&self, name: &str) -> String {
        format!("Hello {}", name)
    }
    // use default impl for greet_loudly
}

impl Greet for Hola {
    fn greet(&self, name: &str) -> String {
        format!("Hola {}", name)
    }
    // override default impl
    fn greet_loudly(&self, name: &str) -> String {
        let mut greeting = self.greet(name);
        greeting.insert_str(0, "¡");
        greeting + "!"
    }
}

#[test]
fn main2() {
    println!("{}", Hello.greet("John")); // prints "Hello John"
    println!("{}", Hello.greet_loudly("John")); // prints "Hello John!"
    println!("{}", Hola.greet("John")); // prints "Hola John"
    println!("{}", Hola.greet_loudly("John")); // prints "¡Hola John!"
}