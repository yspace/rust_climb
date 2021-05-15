static GLOBAL: i32 = 0 ; // 静态变量 必须在声明时马上初始化&&初始化必须是编译器可确定的常量
// mut 修饰的全局变量在使用的时候必须使用unsafe

pub fn main(){
    // 声明变量
    let variable : i32 = 100 ;

//    let _ = variable ;

    // 可写变量
    let mut x = 5 ;
    x = 10 ;
    println!("x = {} ", x) ;

    // 模式
    let (mut a , mut b) = (1,2) ;
//    let Point{x: ref a, y: ref b} = p;
    {
        // 变量遮蔽 shadowing
        let x = "hello" ;
        println!("x is {}", x);

        let x = 5 ;
        println!("x is {}", x);

        let mut v = Vec::new() ;
        v.push(1);
        v.push(2);
        v.push(3);

        let v = v ; // 从此以下v变为只读了 无法再访问上面可写的v了
        for i in &v {
            println!("{}", i) ;
        }

        // ##
        let v = Vec::new() ;
        let mut v = v ;
        v.push(1);
        println!("{:?}", &v);
//        println!("{:?}", v);

    }

    {
        // ## 类型推导
        let elem = 5u8 ; // 通过后缀推导类型

        let mut vec = Vec::new() ;
        vec.push(elem) ; // 通过操作参数类型推导向量类型
        println!("{:?}", vec) ;

        // ## 部分推导
        let player_scores = [
            ("jack",20),
            ("jane",23),
            ("jill",18),
            ("john",19),
        ];

        // 动态数组  成员类型交由编译器推导
        let players : Vec<_> = player_scores
            .iter()
            .map(|&(player, _score)|{
//                player
                (player,"大坏蛋")
            })
            .collect() ;
        println!("{:?}", players);
    }

    {
        // ## 类型别名
        type Age = u32 ;

        fn grow(age : Age , year: u32) ->Age {
            age + year
        }

        let x : Age = 20 ;
        println!("20 years later: {}", grow(x, 20));

        // 泛型场景
        type Double<T> = (T, Vec<T>) ; // 可以简化代码
    }

    // ## 静态变量
    {
        let x ;
        let y = 1_i32 ;
        x = 2_i32 ; // 延迟初始化 只要确保使用前初始化了就行
        println!("{} {}", x, y) ;

        // 全局变量声明时必须初始化 ，因为全局变量可写在函数外面被任意函数使用
        static G1: i32 = 3 ;

        // 可变全局变量 无论读写都必须使用unsafe
        static mut G2 : i32 = 4 ;
        unsafe{
            G2 = 5 ;
            println!("G2  is {}", G2) ;
        }

        // 全局变量的内存不是分配在当前函数栈上的， 函数退出的时候 并不会销毁全局变量占用的内存空间
        // 程序退出时才会回收

        // 允许
        static array: [i32; 3] = [1,2,3] ;
        // 不允许
//        static vec: Vec[i32] = {let mut v = Vec::new() ; v.push(1); v} ;

        // RUST不允许用户在main前后执行自己的代码 所以比较复杂的static变量的初始化一般使用lazy的方式
        // 在第一次使用的时候初始化 rust中较复杂的全局变量初始化推荐使用： lazy_static

    }

    // ## 常量
    {
        const GLOBAL: i32 = 0 ;

        println!("常量{}", GLOBAL) ;
        // 。常量的初始化表达式也一定要
        //是一个编译期常量，不能是运行期的值。它与static变量的最大区别在
        //于：编译器并不一定会给const常量分配内存空间，在编译过程中，它很
        //可能会被内联优化。因此，用户千万不要用hack的方式，通过unsafe代
        //码去修改常量的值，这么做是没有意义的。以const声明一个常量，也不
        //具备类似let语句的模式匹配功能。
    }

}


fn test(condition: bool){
    let x: i32;
    if condition {
        x  = 1 ;
        println!("{}", x) ;
    }
    // 条件不满足不会初始化x
    // 但是只要不再使用x 就没事
}