pub fn run() {
    // more_powerful_destruct() ;
    // ignore_memry_mgr();
    // range_multiple_match() ;
    ref_ref_mut();
}

pub fn basic() {
    // 模式，是Rust另一个强大的特性。它可以被用在let和match表达式里面。
    let tup = (0u8, 1u8);
    let (x, y) = tup;

    // 而且我们需要知道的是，如果一个模式中出现了和当前作用域中已存在的同名的绑定，那么它会覆盖掉外部的绑定

    let x = 1;
    let c = "c";

    match c {
        // 此处外部x被 内部的同名x覆盖
        x => println!("x: {}, c: {}", x, c),
    }

    println!("x: {}", x);
}

fn more_powerful_destruct() {
    struct Point {
        x: i64,
        y: i64,
    }

    let point = Point { x: 1, y: 2 };

    match point {
        Point { x, y } => println!("({}, {})", x, y),
    }
    // 结构字段重命名
    match point {
        Point { x: x1, y: y1 } => println!("({}, {})", x1, y1),
    }

    // 只对特定字段感兴趣
    match point {
        Point {
            y,
            // ..来省略其他字段。
            ..
        } => println!("y is {}", y),
    }
}

fn ignore_memry_mgr() {
    let tuple: (u32, String) = (5, String::from("five"));

    let (x, s) = tuple;

    // // 以下行将导致编译错误，因为String类型并未实现Copy, 所以tuple被整体move掉了。
    // println!("Tuple is: {:?}", tuple) ;

    let tuple = (5, String::from("five"));
    // 忽略String u32实现了Copy
    let (x, _) = tuple;
    println!("Tuple is {:?}", tuple);
}

fn range_multiple_match() {
    // range match
    let x = 1;

    match x {
        // 数字（字符）范围 可以用 ...
        1...10 => println!("1 - 10"),
        _ => println!("others"),
    }

    let c = 'w';
    match c {
        'a'...'z' => println!("lower  "),
        'A'...'Z' => println!("upper"),
        _ => println!("others"),
    }

    // multiple match
    let x = 1;
    match x {
        // use ｜ to  separate multiple pattern conditions
        1 | 2 => println!("1 or 2"),

        _ => println!("other number"),
    }
}

fn ref_ref_mut() {
    //
    let mut x = 5;
    match x {
        ref mut mr => println!("mut ref: {}", mr),
    }
    println!("x is {}", x);

    let ref mut mrx = x;
    // println!("x is {}", x) ;
    println!("mrx is {}", mrx);
    // *mrx = 4 ;

    let maybe_name = Some(String::from("Alice"));
    // Using `ref` , the value is borrowed , not moved ...
    match maybe_name {
        Some(ref n) => println!("Hello {}", n),
        _ => println!("can't be happened "),
    }
    // ... so it's available here!
    println!("hello again , {}", maybe_name.unwrap_or("world".into()));

    #[derive(Debug)]
    struct Person {
        name: String,
    }
    let mut person = Person {
        name: String::from("hello"),
    };
    match person {
        Person { name: ref mut name } => {
            name.push_str(",world");
            println!("{:?}", name);
        }
    }
    println!("{:?}", person);
}
