// Clone 是Copy的父类  要实现Copy 必须实现Clone！
// Copy 是个空接口 起Marker作用
// Copy是隐式发生的 比如 let b = a ; 如果a是实现了Copy的 此时的=操作 就发生了Copy
// Copy 是bitwise copy 对于指针成员就麻烦了（只是拷贝变量的值 指针的话不会考贝其所指 等价问题就是其他语言中的 浅拷贝深拷贝问题） 会导致二次释放问题。
//  String实现了clone 没有实现Copy 这就是我们为什么可以实现clone 而有时不能实现Copy的原因
// 共享类型 &T 也是可拷贝的 不用考虑T类型的具体情况     Vec<T> 是不可考呗的
// 在实现了Drop的类型上不可以实现Copy -- 导致二次drop

// 额外实现者
// - Function items
// - Function pointer types
// - Array types 当其ItemType实现了Copy时
// - Tuple Types 当其组件类型都实现了Copy时
// - Closure 类型 当其捕获的环境变量是共享型变量时   注意&mut T 是互斥的

#[derive(Copy,Clone)]
struct Something ;

struct Something2;
impl Copy for Something2{} 
impl Clone for Something2{
    fn clone(&self) -> Self {
        Self {  }
    }
}

// 可以用这种声名式实现Copy Clone 的前提是其子成员是实现了此二traits的
#[derive(Copy,Clone)]
struct Something3{

    thing: Something ,
}

pub fn run() {

}