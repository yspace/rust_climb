fn run() {
    let mut n = 1;

    loop {
        if n > 101 {
            break;
        }

        n += 1;
    }
}

fn _while() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

fn match_expr() {
    let number = 42;
    match number {
        // 模式为单个值
        0 => println!("Origin"),
        // 模式为Range
        1...3 => println!("All"),
        // 模式为 多个值
        5 | 7 | 13 => println!("Bad Luck"),
        // 绑定模式，将模式中的值绑定给一个变量，供右边执行代码使用
        n @ 42 => println!("Answer is {}", n),
        // _ 通配符处理剩余情况
        _ => println!("Common"),
    }
}

fn while_let() {
    let mut v = vec![1, 2, 3, 4, 5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    //

    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
