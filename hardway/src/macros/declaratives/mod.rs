#![feature(trace_macros)]

pub mod optional_params;

// 很像match语句
macro_rules! noop_macro {
    () => {};
}

macro_rules! print_what_it_is {
    () => {
        println!("A macro with no arguments")
    };
    ($e:expr) => {
        println!("A macro with an expression")
    };
    ($s:stmt) => {
        println!("A macro with a statement")
    };
    ($e:expr, $s:stmt) => {
        println!("An expression followed by a statement")
    };
}

macro_rules! special_println {
    // tt token trees 可以包含一个identifier 或者一系列token trees ； 这里就有递归意味了
    // $() 这个是重复规则
    // * 表示可以重复多次 如同正则式的那种通配符出现数 +是最少一次 *是任意次（包括不出现）?是0或者1次
    ($($arg:tt)*) => {
       println!($($arg)*);
    };
}
macro_rules! special_println2 {
    ($($arg:tt)*) => {
       println!("Printed specially: {}",$($arg)*);
    };
}

#[test]
fn test_noop_macro() {
    noop_macro!();
}

#[test]
fn test_what_it_is() {
    print_what_it_is!();
    print_what_it_is!({});
    print_what_it_is!(;);
    print_what_it_is!(2+3,;);
}

macro_rules! var_print {
    ($($v:ident),*) => {
    println!(concat!($(concat!(stringify!($v),"={:?} ")),*), $($v),*)
};
}

#[test]
pub fn run() {
    // trace_macros!(true);
    special_println2!("hello world!");
    // trace_macros!(false);

    let v1 = "hi";
    let v2 = 2;
    var_print!(v1, v2);
}

mod dont_repeat_yourself {

    
    macro_rules! user_struct {
        ($user:ident) => {
            #[derive(Debug)]
            struct $user {
                name: String,
                age: i32,
                kind: String,
            }
            impl $user {
                fn new(name: &str, age: i32) -> Self {
                    Self {
                        name: name.into(),
                        age,
                        kind: stringify!($user).into(),
                    }
                }
               
            }
        };
    }

    user_struct!(Teacher);
    user_struct!(Student);
    user_struct!(Admin);

    #[test]
    fn test_user_struct() {
        let t = Teacher::new("mr qing".into(),18);
        println!("{:?}", t);
    }
}

mod dont_repeat_yourself2 {
    // 用来对成员变量做约束
    trait User {
        fn name(&self) -> &String; 
        fn age(&self) -> i32;
        fn kind(&self) -> &String;
    }

    macro_rules! user_struct {
        ($user:ident) => {
            #[derive(Debug)]
            struct $user {
                name: String,
                age: i32,
                kind: String,
            }
            impl $user {
                fn new(name: &str, age: i32) -> Self {
                    Self {
                        name: name.into(),
                        age,
                        kind: stringify!($user).into(),
                    }
                }
               
            }
            impl User for $user {
                fn name(&self) -> &String {
                     &self.name
                }
                fn age(&self) -> i32 {
                self.age }
                fn kind(&self) -> &String { 
                    &self.kind
                } 
            }

        };
    }

    user_struct!(Teacher);
    user_struct!(Student);
    user_struct!(Admin);

    #[test]
    fn test_user_struct() {
        let t = Teacher::new("mr qing".into(),18);
        println!("{:?}", t);
        println!("user>> {} {} {}",t.name(),t.age(), t.kind());
    }
}
