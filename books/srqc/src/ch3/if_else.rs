pub fn main(){
    func(1);
    func(0);
    func(-1);

    as_expression() ;
}

fn func(i: i32)->bool{
    if i < 0{
        println!("{} is negative", i);
    }else if i > 0 {
        println!("{} is positive", i)
    }else{
        println!("{} is zero", i) ;
    }

    true
}

fn as_expression(){
    let cond = true ;
    // 注意 if - else 分支表达式的最后类型应该一致哦
    let x: i32 = if cond {
        1
    }else{
        // 如果else分支省略掉了，那么编译器会认为else分支的类型默认为（）
        2
    };
    println!("x is {}", x) ;
}