// @see https://github.com/actix/actix-web/blob/master/actix-web/src/handler.rs

// @see https://docs.rs/runestick/0.6.16/src/runestick/module.rs.html#667-684


pub trait Handler<Args>: Clone + 'static {
    // type Output;
    // type Future: Future<Output = Self::Output>;

    // fn call(&self, args: Args) -> Self::Future;
    fn call(&self, args: Args) -> (); // 先没有返回值
}

pub trait MyFnTrait<Args>: Clone + 'static {
      fn call(&self, args: Args) -> ();
}

#[test]
fn test_impl_my_fn_trait() {
    // 也可以不引入T 直接替换到for Fn(P1,P2) ;就是直接为函数trait实现其他trait！ 这个地方用到的是覆盖实现 generic blanket impls
    // https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#generic-blanket-impls
    impl<T, P1,P2> MyFnTrait<(P1,P2)> for T
        where
        T: Fn(P1,P2) -> ()+ Clone + 'static,
        {
    
            #[inline]
            #[allow(non_snake_case)]
            fn call(&self, (P1,P2): (P1,P2)) /*-> Self::Future*/ {
                println!("<< 基于trait 可以为函数 实现额外的方法 比如中间件之类？!");
               let _result =  (self)(P1,P2);
                println!("hahahha />>");
                // println!("{:?}", P1);
                // println!("{:?}", P2);
            }
        }
        fn assert_impl<T>(_: impl MyFnTrait<T>) {}
        
        fn call_my_fn_trait(f: impl MyFnTrait<(String,String)>) {
            let args = ("hello".to_owned(), "world".to_owned());
            
            f.call(args)
        }

        fn some_fn(p1:String, p2:String){
            println!("some_fn is called");

            println!("{} {}", p1, p2);
        }

        assert_impl(some_fn);

        call_my_fn_trait(some_fn);
}


/// Generates a [`Handler`] trait impl for N-ary functions where N is specified with a sequence of
/// space separated type parameters.
///
/// # Examples
/// ```ignore
/// factory_tuple! {}         // implements Handler for types: fn() -> R
/// factory_tuple! { A B C }  // implements Handler for types: fn(A, B, C) -> R
/// ```
// macro_rules! factory_tuple ({ $($param:ident)* } => {
//     impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
//     where
//         Func: Fn($($param),*) -> Fut + Clone + 'static,
//         Fut: Future,
//     {
//         type Output = Fut::Output;
//         type Future = Fut;

//         #[inline]
//         #[allow(non_snake_case)]
//         fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future  {
//             (self)($($param,)*)
//         }
//     }
// });
macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> ()+ Clone + 'static,
    {
       

        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) /*-> Self::Future*/ {
            (self)($($param,)*)
        }
    }
});

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
factory_tuple! { A B C D E F G H I }
factory_tuple! { A B C D E F G H I J }
factory_tuple! { A B C D E F G H I J K }
factory_tuple! { A B C D E F G H I J K L }
factory_tuple! { A B C D E F G H I J K L M }
factory_tuple! { A B C D E F G H I J K L M N }
factory_tuple! { A B C D E F G H I J K L M N O }
factory_tuple! { A B C D E F G H I J K L M N O P }

#[cfg(test)]
mod tests {
    use super::*;

    // 为不同的元组实现FromRequest： https://github.com/actix/actix-web/blob/19c9d858f25e8262e14546f430d713addb397e96/actix-web/src/extract.rs#L315

    // fn assert_impl_handler<T: FromRequest>(_: impl Handler<T>) {}
    fn assert_impl_handler<T>(_: impl Handler<T>) {}

    #[test]
    fn arg_number() {
         fn handler_min() {}

        #[rustfmt::skip]
        #[allow(clippy::too_many_arguments, clippy::just_underscores_and_digits)]
       fn handler_max(
            _01: (), _02: (), _03: (), _04: (), _05: (), _06: (),
            _07: (), _08: (), _09: (), _10: (), _11: (), _12: (),
            _13: (), _14: (), _15: (), _16: (),
        ) {}

        assert_impl_handler(handler_min);
        assert_impl_handler(handler_max);
    }

    // 对T 总要有些约束才好
    fn assert_impl_handler2<T>(_: impl Handler<T>) {}
    #[test]
    fn arg_number_for_str() {
         fn handler_min(_01: String,_02: &str) {}

    //     #[rustfmt::skip]
    //     #[allow(clippy::too_many_arguments, clippy::just_underscores_and_digits)]
    //    fn handler_max(
    //         _01: String ,_02: String, _03: String, _04: String, _05: String, _06: String,
    //         _07: String, _08: String, _09: String, _10: String, _11: String, _12: String,
    //         _13: String, _14: String, _15: String, _16: String,
    //     ) {}

        assert_impl_handler2(handler_min);
        // assert_impl_handler2(handler_max);
    }
}
