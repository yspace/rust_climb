pub fn main(){
    func(1);
    func(0);
    func(-1);

    as_expression() ;

    learn_loop() ;
    loop_as_expression() ;
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

fn learn_loop(){
    let mut count = 0u32 ;
    println!("Let's count util infinity!");

    // 无限循环
    loop{
        count += 1 ;
        if count == 3{
            println!("three");
            // 跳过本次循环 下面的不再执行
            continue ;
        }

        println!("{}", count);
        if count == 5 {
            println!("OK that's enought");

            // 跳出循环
            break ;
        }
    }

}

fn learn_loop2(){

    let mut m = 1 ;
    let n = 1 ;

    // 加生命周期 标识循环块  以便内部可以用continue break 跳过或者终结循环
    'a: loop{
        if m<100{
            m+=1 ;
        }else{
            'b: loop{
                if m+n > 50 {
                    println!("break") ;
                    break 'a ;
                }else{
                    continue 'a ;
                }
            }
        }
    }



}

fn loop_as_expression(){
    let v = loop {
        break 10 ;
    };
    println!("{}", v) ;

    // 发散类型  死循环？
    // let v = loop {};
    // println!("{:?}", v);
}