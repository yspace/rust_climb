// ## display formats
// {} | {:?} | {value} | {:05} 五对齐 不够补零 ｜{:4} 四个空格 ｜{:#?} pretty print
// {somevalue:some-format} 冒号前面的是对要现实对象的引用 没有就按照位置递增 
pub fn run() {

    // ## 
    println!("{:1$}!",'x' , 5);
    println!("{0:1$}!",'x' , 5);

    let s = format!("{:1$}",'x',5);
    let s = format!("{:width$}",'x',width = 5);

    let obj = 55 ;
    for spaces in 1..10 {
        println!("{obj:width$} !",  width = spaces);
        println!("{obj:00$} !",   spaces); // 注意两个0 的意义不同第一个是前面补足 第二个是位置指代0$
        // println!("{obj:00$} !{:1$}", 22,   spaces); // 注意两个0 的意义不同第一个是前面补足 第二个是位置指代0$
    }

    // ## 填充与对齐 :[fill][align-sign]    对齐符号：< ^ > 分别代表左中右
    println!("hello {:<5}!", "x"); // 左duiqi
    println!("hello {:*<5}!", "x"); // 左对齐 并填充够5
    println!("hello {:>5}!", "x"); // 右对齐
    println!("hello {:^5}!", "x"); // 中对齐

    // % 填充 右对齐 动态指定对其到8位
    println!("hi {:%>width$}!",0 , width = 8);
    
    // https://docs.rs/emojis/0.1.1/emojis/struct.Emoji.html
    // Unicode.org 
    let fill_unicode = '🚀';

    println!("hi {:🚀>width$}!",0 , width = 8);

    for spaces in -10 .. 10 {
        // 这些符号组合起来 会疯的^-^
        println!("{:*^-#width$?}!",(spaces,spaces),width=9);
        println!("{:*^-#width$.5?}!",(spaces,spaces as f32),width=9);
    }

    // ## escape
    println!("{{}}" ) ;

    // ## 进制 o|x?|b|e     
    println!("{:X?}",100) ;
    println!("{:o}",100) ;
    // println!("{:0}",100) ;
    println!("{:b}",100) ;
    println!("{:e}",1000) ;

    // 指针
    println!("{:p}", &100);

}

