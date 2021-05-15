

pub fn main(){
    // Rust在内存安全方面的设计方案的核心思想是“共享不可变，可变
// 不共享”。

    let i = 0;
    let p1 = & i;
    let p2 = & i;
    println!("{} {} {}", i, p1, p2);

    //
    let mut i = 0;
    let p1 = &mut i;
    *p1 = 1;

    // 
    {
        use StringOrInt::{Str, Int};
        let mut x = Str("Hello world".to_string());
        // if let Str(ref insides) = x {
        //     x = Int(1);
        //     println!("inside is {}, x says: {:?}", insides, x);
        // }
        // 上面if语句中 出现 既读又写的现象

        x = Int(1);
        let _ = x ;
    }

    {
        // 迭代器失效
        // 在遍历容器的时候同时对容器做修改，可能出现在多线程场景，也可能出现在单线程场景
        let mut arr = vec![
            "ABC",
            "DEF",
            "GHI",
        ];
        for item in &arr {
            // arr.clear() ;  // 已经读借用了 就不能再写了
        }

    }

    {
        let mut arr : Vec<i32> = vec![1,2,3,4,5];
        let p : &i32 = &arr[0];
        for i in 1..100 {
            // arr.push(i);  // 不能执行写操作  会触发扩容  然后内存地址变更 导致读引用失效
        }

        // println!("{}", p) ;
    }
}



use std::fmt::Debug;
#[derive(Debug)]
enum StringOrInt {
        Str(String),
        Int(i64),
}