#![feature(const_fn)] // 常量函数特性

pub fn main(){
    println!("1+2 = {}", add1((1, 2))) ;
    println!("1+2 = {}", add2((1, 2))) ;
    first_class() ;

    fn_types() ;
    test_inner();

    read_env_vars();
    const_func();

    recusive_fn() ;
}

fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}
// 函数的参数列表与let语句一样，也是一个“模式解构”。
fn add2((x, y) : (i32, i32) ) -> i32 {
    x+y
}

// 函数如头等公民一样(其他变量一样？) 被传递 赋值 调用
fn first_class(){
    let p = (1,3) ;

    let func = add2 ;
    println!("evaluation output {}", func(p)) ;
}

fn fn_types(){
    /*
    // 函数也是特殊类型：
    // 先让 func 指向 add1
    let mut func = add1;
// 再重新赋值,让 func 指向 add2
    func = add2; // 注意这里的 fn item 根据编译器的指示
    */
    // 修复方案一： 让func的类型为通用的fn类型
    {
        // 写法一,用 as 类型转换
        let mut func = add1 as fn((i32,i32))->i32;
// 写法二,用显式类型标记
        let mut func : fn((i32,i32))->i32 = add1;
    }

    // 方案二 使用trait
    {
        let mut func : &dyn Fn((i32, i32))-> i32 = &add1 ;
        println!("result: {}", func((1,2)) );
        func = &add2 ;
        println!("result: {}",func((1,2))) ;
    }

}

// ，Rust的函数体内也允许定义其他item，包括
// 静态变量、常量、函数、trait、类型、模块等
fn test_inner(){
    // NOTE 当你需要一些item仅在此函数内有用的时候，可以把它们直接定义到函数体内，以避免污染外部的命名空间。

    static  INNER_STATIC : i64 = 42 ;

    fn internal_incr(x: i64) ->i64 {
        x + 1
    }

    struct InnerTemp(i64) ;
    impl InnerTemp{
        fn incr(&mut self) {
            self.0 = internal_incr(self.0) ;
        }
    }

    // 函数体 执行语句
    let mut t = InnerTemp(INNER_STATIC) ;
    t.incr() ;
    println!("{}", t.0 );
}
// 在Rust中，有以下这些情况永远不会返回，它们的类型就是！。
//·panic！以及基于它实现的各种函数/宏，比如unimplemented！、
//     unreachable！；
//·死循环loop{}；
//·进程退出函数std：：process：：exit以及类似的libc中的exec一类
//     函数。
//
// 与任意类型相容
fn diverging_funcs(){
    fn diverges() -> ! {
        panic!("This function never returns!");
    }
    let x : i32 = diverges();
//   。发散类型的最大特点就是，它可以被转换为任意一个类型
    let x : i32 = diverges();
    let y : String = diverges();

    // 原因
    let p = if true {
        panic!("error");
    } else {
        100
    };
}

fn _main() {
 /**
//   以C语言为例，主函数的原型一般 允许定义成以下几种形式：
int main(void);
int main();
int main(int argc, char **argv);
int main(int argc, char *argv[]);
int main(int argc, char **argv, char **env);
*/
    // rust 中main 函数的参数传递 和状态码

 //. 每个被空格分开的字符串都是一个参数。进程可以在任何时候调用
 //  exit（）直接退出，退出时候的错误码由exit（）函数的参数指定
     fn main() {
         for arg in std::env::args() {
             println!("Arg: {}", arg);
         }
         std::process::exit(0);
     }
}

// 此例根据命令行传递的参数来打印对应的环境变量
//   cargo run -p srqc ch4 PATH
fn read_env_vars(){
    for arg in std::env::args() {
        match std::env::var(&arg) {
            Ok(val) => println!("{}: {:?}", &arg,  val) ,
            Err(e) => println!("couldn't find environment {}, {}" , &arg , e),
        }
    }

    println!("All environment varible count: {}", std::env::vars().count()) ;
}


const fn  cube(num: usize) -> usize{
    num * num * num
}

fn const_func(){
    const DIM: usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM] ;

    println!("{:?}", ARR) ;
}

// 递归函数
fn fib(index: u32) ->u64 {
    if index == 1 || index == 2 {
        1
    }else{
        fib(index - 1) + fib(index -2)
    }
}

// 当前版本的Rust暂时还不支持尾递归优化，因此如果递归调
// 用层次太多的话，是有可能撑爆栈空间的。
fn recusive_fn(){
    let f8 = fib(8);
    println!("fib result: {}", f8) ;
}