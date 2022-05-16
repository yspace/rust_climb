use std::collections::HashMap;

mod fn_types ;
mod pass_by_ref ;


pub fn main() {
    // func_params::run() ;
    // return_value::run() ;

    // higher_order_func::run();


    fn_types::call_it(||{println!("it works !")});
    
    {
        let mut some_var = 2 ;
        fn_types::call_it2(Box::new(move||{
            println!("it works !");
            println!("val: {}",some_var);
        }));  
    }
    let f_foo = || { println!("foo closure is called"); } ;
    fn_types::is_Action(f_foo) ;
    fn_types::call_it(f_foo) ;
   
}

mod func_params {
    use super::*;

    pub fn run() {
        // 在rust中，函数是一等公民（可以储存在变量/数据结构中，可以作为参数传入函数，
        // 可以作为返回值），所以rust的函数参数不仅可以是一般的类型，也可以是函数
        say_what("xiaoli", hi);
        say_what("xiaoMing", hello);

        // 存储在数据结构中
        fn foo() {
            println!("call foo!");
        }
        fn bar() {
            println!("call bar!");
        }
        let mut kn = KnowledgeNode::new("foo", foo);
        println!("kn: {:?}", kn);
        kn.func = bar;
        println!("kn: {:?}", kn);
        kn.func = foo;

        let mut fm = HashMap::new();
        fm.insert("foo", kn);
        let kn2 =
            KnowledgeNode::new("bar", bar).set_description(Some(String::from("hi this is desc")));
        fm.insert("bar", kn2);
        for name in fm.keys() {
            println!("\n===== call {} ==========", name);
            let node = fm.get(name).unwrap();
            println!("name: {} ", node.name);

            if let Some(desc) = &node.description {
                println!("description:{} ", desc);
            };

            let f = &node.func; // why can't directly call it: &node.func()
            f();

            println!("===== end call {}/=== \n ", name);
        }

        pattern_match();
    }

    fn hi(name: &str) {
        println!("hi {}", name);
    }
    fn hello(name: &str) {
        println!("hello {}", name);
    }

    fn say_what(name: &str, func: fn(&str)) {
        func(name);
    }

    fn pattern_match() {
        fn print_id((name, age): (&str, i32)) {
            println!(" i am {} and i am {} years old .", name, age);
        }

        // 参数的模式匹配跟let语句的匹配一样，也可以使用下划线来表示丢弃一个值
        fn print_age((_, age): (&str, i32)) {
            println!("My age is  {} ;", age);
        }

        let qing = ("yiqing", 18); // ^-^ I am always 18 !
        print_id(qing);
        print_age(qing);
    }
}

/// Language points; Knowledge Points; Knowledge;
#[derive(Debug)]
struct KnowledgeNode {
    name: String,
    description: Option<String>,
    func: fn(),
}

impl KnowledgeNode {
    pub fn new(name: &str, func: fn()) -> Self {
        Self {
            name: String::from(name),
            func,
            description: None,
        }
    }

    pub fn set_description(mut self, desc: Option<String>) -> Self {
        self.description = desc;
        self
    }
}

mod return_value {
    pub fn run() {
        let mut f: fn();
        f = f1;
        f = f2;
        println!("f1 and f2 are same type!");

        println!(" 3+1 = {}", inc(3));

        // return
        return_keyword();
        return_multiple_values();
        divergin_func();
    }

    fn f1() {}
    fn f2() -> () {}

    fn inc(n: i32) -> i32 {
        n + 1 // expression is return statement !
    }

    fn return_keyword() {
        fn find(n: i32, a: &[i32]) -> bool {
            // slice can be treated as ref of array
            for i in a {
                // i type is &i32 , use dereference to access the real int value
                if *i == n {
                    return true;
                }
            }

            false
        }

        let a = [1, 3, 2, 5, 9, 8];
        println!("there is 7 in the array: {}", find(7, &a));
        println!("there is 8 in the array: {}", find(8, &a));
    }

    fn return_multiple_values() {
        // rust的函数不支持多返回值，但是我们可以利用元组来返回多个值，配合rust的模式匹配，使用起来十分灵活
        fn pow_2_3(n: i32) -> (i32, i32) {
            (n * n, n * n * n)
        }

        let (p2, p3) = pow_2_3(789);
        println!(" pow 2 of 789 is {}", p2);
        println!(" pow 3 of 789 is {}", p3);
    }

    fn divergin_func() {
        // 发散函数（diverging function）是rust中的一个特性。发散函数不返回，它使用感叹号!作为返回类型表
        fn divergin() -> ! {
            panic!("this function will never retrun !");
        }

        println!("hello");
        divergin();
        println!("world");

        //
        fn divergin2() -> ! {
            loop {}
        }
    }
}

mod stmt_expr {
    pub fn run() {}

    fn declare_stmt() {
        let a = 8;
        let b: Vec<f64> = Vec::new();
        let (a, c) = ("hi", false);
    }
}

mod higher_order_func {
    pub fn run() {
        // 关键字fn可以用来定义函数。除此以外，它还用来构造函数类型。与函数定义主要的不同是，构造函数类型不需要函数名、参数名和函数体

        // function declaration
        fn inc(n: i32) -> i32 {
            n + 1
        }

        type IncType = fn(i32) -> i32; // func type

        let f: IncType = inc; // no moving happen here!

        assert_eq!(3, inc(2));
        println!("inc 3 is: {}", inc(3));
        println!("{:?}", f);

        //
        func_as_param();
        func_as_return_value();
    }

    fn func_as_param() {
        fn inc(n: i32) -> i32 {
            n + 1
        }
        fn dec(n: i32) -> i32 {
            n - 1
        }

        fn process(n: i32, func: fn(i32) -> i32) -> i32 {
            func(n)
        }

        assert_eq!(3 + 1, process(3, inc));
        assert_eq!(3 - 1, process(3, dec));

        // 不过，这不是函数作为参数的唯一声明方法，使用泛型函数配合特质（trait）也是可以的，因为rust的函数都会实现一个trait:FnOnce、Fn或FnMut。将上例中的process函数定义换成以下形式是等价的：
        fn process2<F>(n: i32, func: F) -> i32
        where
            F: Fn(i32) -> i32,
        {
            func(n)
        }

        assert_eq!(3 + 1, process2(3, inc));
        assert_eq!(3 - 1, process2(3, dec));
    }

    fn func_as_return_value() {
        fn get_func(n: i32) -> fn(i32) -> i32 {
            fn inc(n: i32) -> i32 {
                n + 1
            }
            fn dec(n: i32) -> i32 {
                n - 1
            }

            // 不过，在函数中定义的函数，不能包含函数中（环境中）的变量，若要包含，应该闭包
            if n % 2 == 0 {
                inc
            } else {
                dec
            }
        }

        // call it
        let a = [1, 2, 3, 4, 5, 6, 7];
        let mut b = Vec::<i32>::new();
        for i in &a {
            b.push(get_func(*i)(*i));
        }
        println!("{:?}", b);
    }
}
