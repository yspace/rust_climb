// 官方文档中，impl是限定泛型的语法糖

trait Fly {
    fn fly(&self) -> bool;
}
struct Duck;
impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}

fn foo() -> impl Fly {
    Duck
}

pub fn main() {
    let fly_instance = foo();
    if fly_instance.fly() {
        println!("yes it can fly");
    }
}

mod args_returns {
    // 在参数位置 impl Trait 在语义上很像范型类型参数
    // 但在返回值位置 二者有巨大差别 不像使用泛型类型参数  用impl Trait时 函数选择返回类型
    // 调用者无法选择返回类型

    trait Trait {}
    struct MyStruct() ;
    impl Trait for MyStruct{}

    // 允许调用者决定返回类型
    fn foo<T: Trait>(arg: T) {}

    fn foo2(arg: impl Trait) {}

    // 不允许调用者决定返回类型 ，而是由函数选择返回类型 但仅承诺该类型实现Trait
    fn foo3() -> impl Trait {
        MyStruct()
    }

    fn f1(flag: bool) -> impl FnOnce()->usize{
       if flag {
          || 3 
       }else{
         || { println!("hahaha") ; 4}
       }
        
    }
   
    fn f2()-> Box< dyn FnOnce()-> usize> {
        Box::new(|| 3)
    }
}
