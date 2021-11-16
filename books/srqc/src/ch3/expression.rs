pub fn main(){
    learn_ops() ;
    learn_bit_ops();

    learn_logic_ops() ;
    learn_assignment();

    // 测试下哈希字典功能
    _hash() ;
}

fn learn_ops(){
    let x = 100 ;
    let y  = 10 ;
    println!("{} {} {} {} {}",
        x+y ,
        x-y ,
        x*y ,
        x/y,
        x%y
    );
}

fn learn_bit_ops(){
    let num1:u8 = 0b_1010_1010;
    let num2:u8 = 0b_1111_0000;

    println!("{:08b}", !num1);
    println!("{:08b}", num1 & num2);
    println!("{:08b}", num1 | num2);
    println!("{:08b}", num1 ^ num2);
    println!("{:08b}", num1 << 4);
    println!("{:08b}", num1 >> 4);
}

fn learn_logic_ops(){
    fn f1() ->bool{
        println!("Call f1");

        true
    }

    fn f2()-> bool {
        println!("Call f2") ;

        false
    }

    //
    //  ，“逻辑与”、“逻辑或”具备“短路”功能
    println!("Bit and: {}\n", f2() & f1());
    println!("Logic and: {}\n", f2() && f1());
    println!("Bit or: {}\n", f1() | f2());
    println!("Logic or: {}\n", f1() || f2());
}

fn learn_assignment(){
    // 。赋值表达式具有“副作用”：当它执行的时候，会把右边表达式的
    //值“复制或者移动”（copy or move）到左边的表达式中。

    let mut x : i32 = 1 ;

    x = 2 ;

    // 赋值表达式的结果为 unit

    let x = 1;
    let mut y = 2;
// 注意这里专门用括号括起来了
    let z = (y = x);
    println!("{:?}", z);

    // 组合赋值表达式
    let x = 2;
    let mut y = 4;
    y += x;
    y *= x;
    println!("{} {}", x, y);
    // NOTE Rust不支持++、--运算符，请使用+=1、-=1替代。
}

fn learn_stmt_block_expression(){
    let x : () = {} ;
    let x: () = {
      println!("Hello.")
    };

    let y: i32 = {
      println!("Hello.") ;
        5
    };

}

fn _hash(){
    // 根据 k=> v 来存储字符串到函数的映射   这样可以搞个简单的路由出来  根据客户端传递过来的key 调用对应方法
    // 参考rust写操作系统的那个例子 函数的切片是这种类型: &[dyn Fn()]
    /**
    #![feature(custom_test_frameworks)]
    #![test_runner(crate::test_runner)]

    #[cfg(test)]
    fn test_runner(tests: &[&dyn Fn()]) {
        println!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
    }
*/

    use std::collections::HashMap;
    let mut funcs: HashMap<&str, &dyn Fn()> = HashMap::new();

    fn someFn(){
        println!("this is some Func been called") ;
    }
    fn someFn2(){
        println!("this is some Func been called") ;
    }

    funcs.insert("someFn", &someFn) ;
    funcs.insert("someFn2", &someFn2) ;

    let fn_name =  "someFn" ;//String::from("someFn");
    let func = funcs.get(fn_name);

    match func{
        Some(f) =>  f(),
        None => println!("no fn name called {}", fn_name) ,
    }

    let fx = "someKey" ;
//    let dft_fn = ||{
    fn dft_fn(){
        println!("you may be give a wrong key {}", "someKey") ;
    };
//    let f = funcs.entry(fx).or_insert(someFn);
    let f = funcs.entry(fx).or_insert(&dft_fn);
    f() ;

    // 打印下os参数
    //  注意环境变量是env::vars() env::vars_os 是unicode标准的
    use std::env;

// Prints each argument on a separate line
    for argument in env::args_os() {
        println!("{:?}", argument);
    }

    let args : Vec<String> = env::args().collect() ;
    println!("get args {:?}", args) ;
    // 打印下机器的参数：
//    use std::env;
    println!("ARCH = {}", env::consts::ARCH);
    println!("DLL_EXTENSION = {}", env::consts::DLL_EXTENSION);
    println!("DLL_PREFIX = {}", env::consts::DLL_PREFIX);
    println!("DLL_SUFFIX = {}", env::consts::DLL_SUFFIX);
    println!("EXE_EXTENSION = {}", env::consts::EXE_EXTENSION);
    println!("EXE_SUFFIX = {}", env::consts::EXE_SUFFIX);
    println!("FAMILY = {}", env::consts::FAMILY);
    println!("OS = {}", env::consts::OS);
}