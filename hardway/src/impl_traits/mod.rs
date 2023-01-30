
mod basics ;
mod for_arrays;

trait SomeTrait {}

struct SomeStruct;
impl SomeTrait for SomeStruct {}

fn foo() -> impl SomeTrait {
    // 静态分派 隐藏真正类型
    SomeStruct
}

fn foo2() -> Box<SomeTrait> {
    // 动态分派 创建了trait对象
    Box::new(SomeStruct)
}

mod let_s_say {
    trait Trait {
        fn method(&self);
    }

    impl Trait for i32 {
        fn method(&self) {
            println!("i32 impl the Trait");
        }
    }

    impl Trait for f32 {
        fn method(&self) {
            println!("f32 impl the Trait");
        }
    }

    fn foo() -> Box<Trait> {
        // 引入了Box 意味着内存分配
        Box::new(5) as Box<Trait>
    }

    fn use_trait(t: impl Trait) {}

    #[test]
    fn test_impl_trait() {
        // 静态分派 不会创建trait对象
        // We get static dispatch, but we can hide the real type like this.

        fn foo() -> impl Trait {
            // 相当于 -> impl i32
            5
        }
        fn foo2() -> impl Trait {
            // 相当于 -> impl f32
            5.0
        }
        use_trait(foo());
        use_trait(foo2());
    }
}

mod good_for_closures {
    // before
    fn foo0() -> Box<Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    // after
    fn foo() -> impl Fn(i32) -> i32 {
        // No boxing, no dynamic dispatch.
        |x| x + 1
    }
}


mod scenario_retruning_iterator {
    //use std::any::type_name_of_val;

    fn foo() -> impl Iterator<Item = i32> {
        // Each adapter in the chain adds a new type
        // 每个适配器都会增加一个新类型 导致这个方法链最终的返回类型嵌套非常深
        vec![1, 2, 3]
            .into_iter()
            .map(|x| x + 1)
            .filter(|x| x % 2 == 0)
    }

    fn found_type(x:impl Iterator<Item = i32>){
      //println!("{}", type_name_of_val(&x));
    }

    #[test]
    fn test_type() {
        found_type(foo());
    }
}

// 可以使用汇编查看代码生成情况 ，
trait HiTrait{
    fn say_hi(&self) ;
}

struct Dog ;
struct Cat ;

impl HiTrait for Dog {
    fn say_hi(&self){
        println!("wong wong!") ;
    }
}

impl HiTrait for Cat
{
    fn say_hi(&self){
        println!("meow meow!") ;
    }
}
// 这里是静态派发 编译器根据是否调用某方法来生成代码
fn func_impl(arg: impl HiTrait){
    arg.say_hi();
}
// 动态分发 代码只生成一次
fn func_dyn(arg: &dyn HiTrait) {
    arg.say_hi();
}
// 以上两种情形是 参数位置是trait的 对于返回值位置是trait的同样适用

fn get_hi() -> impl HiTrait {
    // 只能是静态决策类型 条件式动态决定返回类型的就不行了
  return Dog{}
} 

fn get_hi2()-> Box<dyn HiTrait>{
    let p = 10 ;

    if p > 10 {
        Box::new(Dog{})
    }else{
        Box::new(Cat{})
    }
}