trait Bar {
    type S;
    fn bar(&self) -> Vec<Self::S>;
}

struct Foo{
    name: &'static str,
} 
impl Bar for Foo{
    type S = Self ;
    fn bar(&self) -> Vec<Self> {
        vec![Foo{name:"foo1"},Foo{name:"foo2"}]
    }
}
// impl Bar for Foo { ... }
// impl Bar for Baz { ... }

fn f<B: Bar>(b: B) -> usize{
    b.bar().len()
}

// a shorthand for the above generic version
fn f2(b: impl Bar) -> usize{
    // can also use b: &impl Bar or b: Box<impl Bar>, etc.
    b.bar().len()
}


mod trait_object{
    use super::{Bar, Foo};

    fn f(b: &dyn Bar<S = Foo>) -> usize{
        b.bar().len()
    }

}

mod impl_in_return_position{
    use super::{Foo, Bar};
    //  The key difference is whether the caller or the callee chooses the concrete type.

    // caller chooses the concrete type, therefore the callee must provide functions with any return type that the caller could choose
    // fn f0<T: Bar>() -> T{
    //     Foo{name:"foo"} 
    // }
    // then the callee chooses the concrete type (i.e., the compiler infers the concrete type from the function body).
    // 
    fn f() -> impl Bar{
        // 这种形式不能如同工厂方法那样 由参数返回不同的实现类型 类型必须一样 成员可以不同 因为要在编译期决定大小 不能是动态大小
        Foo{name:"foox"}
    }

    #[test]
    fn main() {
        let b = f();
        let _ = b.bar();
    }
}