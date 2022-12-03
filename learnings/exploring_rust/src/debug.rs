// Debug trait
// 打印时的 `?` 或者dbg! 宏
// `#?` 会打印的更好看一些
// 应该尽可能的derive 而非手动实现

// 最佳实践： 为公共暴露的类型实现Debug

use std::fmt::Debug;


struct Position{
    lon: f32 , // longitude
    lat: f32 , // latitude
}

impl Debug for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Position").field("lon", &self.lon).field("lat", &self.lat).finish()
        f.debug_tuple("Pos")
        .field( &self.lon)
        .field( &self.lat)
        .finish()
    
    }
}

pub fn run(){
    let name = "qing";
    let age = 18 ;
    let s = format!("{} is {} old" , name, age);

    println!("{}",s) ;

    // 用位置引用 注意{}是迭代器输出 碰到一个输出next 此例中有索引引用的方式
    // 格式化字符串必须引用到所有的参数 不然编译错误 就是说必须全覆盖 可以多次引用
    println!("{0} , {} , {1},{},{1}", 2, 3) ;

    // ## named parameters
    println!("NP>: {a}", a=2) ;

    // ## formatting parameters
    println!("hello {:5}!","x") ;
}