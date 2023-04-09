pub mod impl_fn_vs_dyn_fn;

pub fn main() {
    // 闭包是引用了自由变量的函数
    // 在rust中，函数和闭包都是实现了Fn、FnMut或FnOnce特质（trait）的类型。任何实现了这三种特质其中一种的类型的对象，都是 可调用对象

    // closure_syntax::run();

    as_arg_and_return_value::run();
}

mod closure_syntax {
    pub fn run() {
        let plus_one = |x: i32| x + 1;

        assert_eq!(2, plus_one(1));

        let plus_two = |x| {
            let mut result: i32 = x;

            result += 1;
            result += 1;

            result
        };
        assert_eq!(4, plus_two(2));
        // 闭包 可以不用写全 参数类型 和返回值类型 因为没有文档的需求

        fn plus_one_v1(x: i32) -> i32 {
            x + 1
        }
        let plus_one_v2 = |x: i32| -> i32 { x + 1 };
        let plus_one_v3 = |x: i32| x + 1;
        println!("{},{}, {}", plus_one_v1(1), plus_one_v2(1), plus_one_v3(1))
    }

    fn capture_vars() {
        let mut num = 5;
        {
            let plus_num = |x: i32| x + num;
        } // plus_num goes out of scope, borrow of num ends
        let y = &mut num;

        //

        let num = 5;

        let owns_num = move |x: i32| x + num;

        // 那么在这个例子中，我们的闭包取得了一个num的可变引用，然后接着我们调用了add_num，它改变了其中的值，正如我们期望的。
        // 我们也需要将add_num声明为mut，因为我们会改变它的环境。
        let mut num = 5;

        {
            let mut add_num = |x: i32| num += x;

            add_num(5);
        }

        assert_eq!(10, num);

        //
        {
            let mut num = 5;

            {
                let mut add_num = move |x: i32| num += x;

                add_num(5);
            }

            assert_eq!(5, num);
        }
    }
}

mod implementation {
    // Fn获取&self，
    // FnMut获取&mut self，
    // 而FnOnce获取self。
    pub fn run() {}

    mod foo {
        /*
         pub trait Fn<Args> : FnMut<Args> {
             extern "rust-call" fn call(&self, args: Args) -> Self::Output;
         }

         pub trait FnMut<Args> : FnOnce<Args> {
             extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
         }

         pub trait FnOnce<Args> {
             type Output;

             extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
         }
        */
    }
}

mod as_arg_and_return_value {

    pub fn run() {
        // Taking closures as arguments
        // 现在我们知道了闭包是 trait，我们已经知道了如何接受和返回闭包；就像任何其它的 trait！

        // 这也意味着我们也可以选择静态或动态分发。
        fn call_with_one<F>(some_closure: F) -> i32
        where
            F: Fn(i32) -> i32,
        {
            some_closure(1)
        }

        let answer = call_with_one(|x| x + 2);
        assert_eq!(3, answer);

        // 动态分发
        fn call_with_one2(some_closure: &Fn(i32) -> i32) -> i32 {
            some_closure(1)
        }

        // 现在我们取得一个trait对象，一个&Fn
        let answer = call_with_one2(&|x| x + 2);
        assert_eq!(3, answer);

        // 函数指针 和闭包
        fn add_one(i: i32) -> i32 {
            i + 1
        }
        let f = add_one;
        let answer = call_with_one2(&f);
        assert_eq!(2, answer);
        assert_eq!(2, call_with_one2(&add_one));

        //
        returning_closures();
    }

    fn returning_closures() {
        // 每个闭包生成了它自己的环境struct并实现了Fn和其它一些东西，这些类型是匿名的。它们只在这个闭包中存在。所以Rust把它们显示为closure <anon>，而不是一些自动生成的名字。
        // fn factory() -> Box<dyn Fn(i32) -> i32> {
        fn factory() -> Box<Fn(i32) -> i32> {
            let num = 5;

            // 通过把内部闭包添加move关键字，我们强制闭包使用 move 的方式捕获环境变量。因为这里的 num 类型是 i32，实际上这里的 move 执行的是 copy, 这样一来，闭包就不再拥有指向环境的指针，而是完整拥有了被捕获的变量。并允许它离开我们的栈帧。
            Box::new(move |x| x + num)
        }

        let f = factory();

        let answer = f(1);
        assert_eq!(6, answer);
    }
}
