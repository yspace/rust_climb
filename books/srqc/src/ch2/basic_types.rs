
pub  fn main(){
    learn_bool();
    learn_char();

    learn_int() ;
    learn_float() ;
}
fn learn_bool(){

    let x = true ;
    let y: bool = !x ;

    let z = x && y ;
    println!("{}", z) ;

    let z = x || y ;
    println!("{}", z) ;

    let z = x & y ;
    println!("{}", z);

    let z = x | y ;
    println!("{}", z);

    let z = x ^ y ;
    println!("{}", z);

    logical_op(2,3) ;

    //
    let (a, b) = (3, 5) ;
    if a >= b {
        println!("a>=b");
    }else {
        println!("z<b");
    }

    let mut i = 0 ;
    while i <=10 {
        println!("i: {}",i) ;
        i +=1 ;
    }
}

fn logical_op(x: i32, y: i32){
    let z = x<y ;
    println!("{}<{} => {}",x, y, z) ;
}

// 学习char类型
fn learn_char(){
    let love =  '❤';
    println!("love char unicode: {}", love) ;

    // 转义
    let c1 = '\n' ;
    let c2 = '\x7f' ; // 8 bit 字符变量
    let c3 = '\u{7FFF}' ; // unicode字符

    // 因为char类型的设计目的是描述任意一个unicode字符，因此它占据
    // 的内存空间不是1个字节，而是4个字节。
    println!("字节大小：{}",core::mem::size_of_val(&c3));

    // Ascii 字符只需一个字节 可用b在字符或者字符串前面来告诉rust用一个字节来存字符
    let x : u8 = 1 ;
    let y : u8 = b'A';
    let s : &[u8; 5] = b"hello";
    let r : &[u8; 14] = br#"hello \n world"# ;

    let sofv = &core::mem::size_of_val ; // 这鬼用法 跟类型绑住了
    println!("{}, ", sofv(r))  ;
    println!("{},{},{}",
             core::mem::size_of_val(&x),
             core::mem::size_of_val(&y),
             core::mem::size_of_val(&s) )  ;

}

fn learn_int(){
    // Rust原生支持了从8位到128位的整数。需要特别关注的是isize和
    //usize类型。它们占据的空间是不定的，与指针占据的空间一致，与所在
    //的平台相关。

    // 在C++中与它们相对应的类似类型是int_ptr和uint_ptr。

    let v1 :i32 = 32 ; // 十进制
    let v2 : i32 = 0xFF; // 十六进制
    let v3 : i32 = 0o55 ; // 八进制
    let v4 : i32 = 0b1001 ;// 二进制 注意他们前缀哦

    let var5 = 0x_1234_ABCD; // 在任意位置添加下划线 使用下划线分割数字,不影响语义,但是极大地提升了阅读体验。

    // 字面量后跟后缀代表类型 进而可以省略类型声明
    let v6 = 123usize ;
    let v7 = 0x_ff_u8 ;
    let v8 = 32 ; // 默认类型是i32

    // NOTE rust中可为任何类型添加方法 内置的整型也不例外
    let x : i32 = 9 ;
    println!("9 power 3 = {}", x.pow(3))  ;
    println!("9 power 3 = {}", 9_i32.pow(3))  ; // 直接字面量上调用函数

    // 默认类型
    let x = 10 ;
    let y = x * x ;
    println!("{}",y) ;

    {
        // 这个是为整数随便实现一个trait 接下来就能无缝使用  这个功能太牛了
        trait MyPrefix{
            fn prefix(&self,prefix: String)-> String ;
        }
        impl   MyPrefix for i32{
            fn prefix(&self, prefix: String)-> String{
                format!("{}{}",prefix, self)
            }
        }

        println!("前缀功能： {}",  32.prefix("hi".to_string()) );
    }

    {
        // 整数溢出 编译器会帮我们检测的  可以通过编译开关 使用溢出策略
        // 编译时就过了  但运行期就呵呵 （可以给程序埋雷！！！)\
        // -C overflow-checks=no
        fn arithmetic(m: i8, n: i8){
            println!("{}", m+n) ; // 有溢出风险
        }
//
//        let m : i8 = 120 ;
//        let n : i8 = 120 ;
//        arithmetic(m, n);

        // 若想自主控制整数溢出 可以使用标准库提供的函数
        let i = 100_i8 ;
        println!("checked {:?}", i.checked_add(i)) ;
        println!("saturating  {:?}", i.saturating_add(i)) ;
        println!("wrapping {:?}", i.wrapping_add(i));
        // 可以看到：checked_*系列函数返回的类型是Option<_>，当出现溢
        //出的时候，返回值是None；saturating_*系列函数返回类型是整数，如果
        //溢出，则给出该类型可表示范围的“最大/最小”值；wrapping_*系列函数
        //则是直接抛弃已经溢出的最高位，将剩下的部分返回。在对安全性要求
        //非常高的情况下，强烈建议用户尽量使用这几个方法替代默认的算术运
        //算符来做数学运算，这样表意更清晰。

        {
            // 截断
            use std::num::Wrapping;

            let big  = Wrapping(std::u32::MAX);
            let sum = big + Wrapping(2_u32) ;
            println!("{}",sum.0);
        }
    }

}

fn learn_float(){
    // 在标准库中，有一个std：：num：：FpCategory枚举，表示了浮点
    //数可能的状态

    let f1 = 123.0f64 ;
    let f2 = 0.1f64 ;
    let f3  = 0.1f32 ;
    let f4 = 12E+99_f64 ;
    let f5:f64 = 2. ;

    // TODO :看下 IEEE 754标准对浮点数的定义
    // ##
// 变量 small 初始化为一个非常小的浮点数
        let mut small = std::f32::EPSILON;
// 不断循环,让 small 越来越趋近于 0,直到最后等于0的状态
        while small > 0.0 {
            small = small / 2.0;
            println!("{} {:?}", small, small.classify());
        }
    // 无穷大状态
    let x = 1.0f32 / 0.0;
    let y = 0.0f32 / 0.0;
    println!("{} {}", x, y);
    // 对Infinity做运算 结果不可预料
    let inf = std::f32::INFINITY;
    println!("{} {} {}", inf * 0.0, 1.0 / inf, inf / inf);

    // NAN 非全序
    let nan = std::f32::NAN;
    println!("{} {} {}", nan < nan, nan > nan, nan == nan);
}

fn type_as(){
    // 并非所有类型之间都可互相转换
    let v1 : i8 = 41 ;
    let v2 : i16 = v1 as i16 ; // 转换动作需要显式标记出来 不允许地下党

    let i = 42 ;
    // 先转型为 * const i32 再转 *mut i32 不能一步到位的转换
    let p = &i as *const i32 as *mut i32 ;
    println!("{:p}",p);

    // 对于表达式e as U，e是表达式，U是要转换的目标类型，对于允许的转换 具体请参考书本 有个表格

    // 复杂的类型转换需要使用标准库的From Into等trait

}