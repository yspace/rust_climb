pub fn main() {
    // =   会发生两种情况 或move 或copy

    // 注意 对于复合类型（array tuple ...） copy move 语义由每个成员决定
    // 具有一票否决特点 整体可copy除非每个成员可copy 只要有一个是动态类型｜不可copy 那么整体是move语义

    // &T 一般是copy语义  &mut T 一般是move语义
    println!("copy move");

    let a = 1;
    let b = a; // copy
    println!("a is {}", a);

    let c = [1, 2, 3, 4];
    let c2 = c; // copy
    println!("c is {:?}", c);

    let t = (1, 2.4, [1, 3]);
    let t2 = t; // copy
    println!("t is {:?}", t);

    let v = 1;
    let b = &v;
    let b2 = b; // copy 语义
    println!("b is {:?}", b);

    //
    example_struct::run();
}

fn move_semantics() {
    let a = Box::new(1);
    let a2 = a; // move
                // println!("a is {}", a) ;

    let c = ["1".to_string(), "2".to_string()];
    let c2 = c; // move
                // println!("c is {:?}", c);

    let t = (1, 1.2, "hi".to_string());
    let t2 = t; // move
                // println!("t is {:?}", t) ;

    let mut v = 1;
    let b = &mut v;
    let b2 = b; // move 语义
                // println!("b is {:?}", b) ;
}

mod example_struct {
    #[derive(Debug)]
    struct A; // 0 大小的结构体

    #[derive(Debug, Copy, Clone)]
    struct A2;

    struct Point(u32) ;
    struct Person{
        name: String , // 动态类型不能实现copy
        age: u32 ,
    }
    struct Member{
        name: &'static str ,
        age: u32 ,
    }

    pub fn run() {
        // 结构体 默认不会自动实现copy的 需要手动实现
        {
            let a = A;
            let a2 = a; // move
                        // println!("a is {:?}", a) ;
        }

        {
            let a = A2;
            let a2 = a;
            println!("a is {:?}", a);
        }

        //
        {
            #[derive(Debug, Copy)]
            struct A ;
            impl Clone for A {
                // Copy trait的实现是编译器行为
                fn clone(&self) -> Self {
                    println!("manually implement copy for A") ;
                    *self // 按位复制
                }
            }

            let a = A ; 
            let a2 = a ; // 这里发生了copy 隐式调用编译器版本的clone方法 而不是自己实现的那个
            println!("a is {:?}", a);

            let a3 = a.clone() ;

        }
    }
}
