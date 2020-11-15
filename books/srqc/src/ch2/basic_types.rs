
pub  fn main(){
    learn_bool();
    learn_char();

    learn_int() ;
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

}