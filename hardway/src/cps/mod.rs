pub fn main() {

    naive_factorial::run() ;
}

// the identity function written normally
fn id<T>(x: T) -> T {
    x
}

// in continuation-passing style
fn id2<T>(x: T, cc: fn(T)) {
    cc(x);
}

// Sometimes, calling the current continuation argument ret makes its purpose more obvious
fn id3<T>(x: T, ret: fn(T)) {
    ret(x);
}

mod naive_factorial {
    fn fact(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            n * fact(n - 1)
        }
    }

    fn fact2(n: i32, ret: Box<dyn Fn(i32)>) {
        if n == 0 {
            ret(1);
        } else {
            fact2(
                n - 1,
                Box::new(move |x| {
                // Box::new(  |x| {
                     
                    ret(n  * x);
                }),
            );
        }
    }

    pub fn run() {
        let ret = Box::new(|x| {
            println!("{}", x);
        });
        fact2(5, ret);
    }
}

mod tail_recursive_factorial{

    fn fact(n: i32) -> i32 {
        return tail_fact(n, 1) ;
    }
    fn tail_fact(n:i32, a: i32) -> i32 {
        if (n==0 ) {
            a
        }else{
            tail_fact(n-1, n*a) 
        }
    }

    mod cps{
        type ret_fn = Box<dyn Fn(i32)-> i32>;
        fn fact(n: i32, ret: ret_fn) -> i32 {
            return tail_fact(n, 1, ret) ;
        }

        fn tail_fact(n:i32, a: i32, ret: ret_fn) -> i32 {
            if n==0 {
                ret(a) 
            }else{
                tail_fact(n-1, n*a, ret) 
            }
        }
    }
}


mod exceptions_cps{
    // 注意 cps 需要处理异常情况 那么前一个处理应该有异常传递机制 这里后续处理的函数参数变成Result比较合理了
    // 或者变成双参函数 第二个是异常处理函数 在其他语言中是持续上抛的 通过throw机制
    type ReturnFn = Box<dyn Fn(Result<i32, ()>)>;
}