pub mod ownerships ;

pub fn main(){
    ownerships::main() ;
    destructor() ;
    mannually_drop() ;
}

fn destructor(){
    use std::ops::Drop ;

    struct D(i32);
    impl Drop for D {
        fn drop(&mut self){
            println!("destruct {}", self.0) ;
        }
    }

    //
    let _x = D(1) ;
    println!("construct 1") ;
    {
        let _y = D(2) ;
        println!("construct 2") ;
        println!("exit inner scope") ;
    }

    println!("exit main func") ;
}

fn mannually_drop(){
    // 此函数的实现 就是空函数  不过参数用的是move语义
    use std::mem::drop ;

    // 
    let mut v = vec![1,2,3] ;
    drop(v) ;
   // v.push(4) ;

   // ## 对Copy类型 调用drop 是没有用的：
    let x = 1_i32;
    println!("before drop {}", x);
    drop(x);
    println!("after drop {}", x);

    // ## 变量遮蔽（Shadowing）不会导致变量生命周期提前结束，它不等
// 同于drop。

    use std::ops::Drop;
    struct D(i32);
    impl Drop for D {
        fn drop(&mut self) {
            println!("destructor for {}", self.0);
        }
    }
    let x = D(1) ;
    println!("construct first variable ") ;
    let x = D(2) ;
    println!("construct second variable") ;

    // ## 下划线&析构
    /**
        最后，请大家注意区分，std：：mem：：drop（）函数和std：：
    ops：：Drop：：drop（）方法。
    1）std：：mem：：drop（）函数是一个独立的函数，不是某个类
    型的成员方法，它由程序员主动调用，作用是使变量的生命周期提前结
    束；std：：ops：：Drop：：drop（）方法是一个trait中定义的方法，当
    变量的生命周期结束的时候，编译器会自动调用，手动调用是不允许
    的。
    2）std：：mem：：drop<T>（_x：T）的参数类型是T，采用的是
    move语义；std：：ops：：Drop：：drop（&mut self）的参数类型是
    &mut Self，采用的是可变借用。在析构函数调用过程中，我们还有机会
    读取或者修改此对象的属性。
    
    */
    println!("=============") ;
        let _x = D(1);
        let _  = D(2);
        let _y = D(3);
}