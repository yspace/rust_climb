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