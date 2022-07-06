pub fn main() {
    let collection = [1, 2, 3];
    // 索引访问方式可能的安全性问题 ，可能其他程序也在同时访问此集合
    // for 迭代就不会导致这个问题了
    for i in 0..collection.len() {
        //NOTE 运行时开销 触发边界检查
        println!("{}", collection[i]);
    }

    // continue
    for n in 0..10 {
        if n % 2 == 0 {
            continue;
        }
        println!("{}", n);
    }

    // while
    let mut samples = vec![];

    while samples.len() < 10 {
        let sample = 0;

        samples.push(sample);
    }
    println!("{:?}", samples);

    while_true_incr_count();
    aborting_a_loop();
    matches();
    match_needles() ;
}

fn while_true_incr_count() {
    use std::time::{Duration, Instant};

    let mut count = 0;
    // 表示1秒的时长
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}

fn aborting_a_loop() {
    for (x, y) in (0..).zip(0..) {
        if x + y > 100 {
            break;
        }

        println!("{:?}", (x, y));
    }

    // 从嵌套循环中跳出
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer;
                }
                println!("{:?}", (x, y, z));
            }
        }
    }
}

fn conditional_branch() {
    let n = 123456;
    let description = if is_even(n) { "even" } else { "odd" };
    println!("{} is {}", n, description);

    let n = 654321;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };
    println!("{} is {}", n, description);

    let n = loop {
        break 123;
    };
    println!("{}", n);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn matches() {
    /*
    match does not “fall through” to the next option by default. 
    Instead, match returns immediately when a match is found
    */
    // let item = 100;
    let item = 11;
    match item {
        0 => {
            println!("zero");
        }
        10..=20 => {
            println!("(10, 20]");
        }
        40 | 80 => {
            println!("Ether 40 or 80");
        }
        _ => {
            println!("some other condition");
        }
    }
}

fn match_needles(){
    let needle = 42 ;
    let haystack = [1,1,2,5,14,42,132,429,1430,4862];

    for item in &haystack {
        let result = match item {
            42| 132 => "hit!", 
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}