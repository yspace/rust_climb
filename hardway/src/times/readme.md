
[期间和计算](https://rustwiki.org/zh-CN/rust-cookbook/datetime/duration.html

~~~rust
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

use chrono::{DateTime, Duration, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

fn main() {
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now.checked_add_signed(Duration::weeks(2))
            .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
            .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}



use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
}

~~~

###  统计耗时

https://blog.csdn.net/xiangxianghehe/article/details/102810408

~~~rust
use std::time::Instant; // timer

fn main() {
    let start = Instant::now();
 	
 	//even number range in[0, 4000001)
    let iter = (0..400_0001).filter(|x| x % 2 == 0);
    let res:i64 = iter.sum();
    println!("iter.sum is: {:?}", res);
    
    println!("time cost: {:?} ms", start.elapsed().as_millis());// ms
    println!("time cost: {:?} us", start.elapsed().as_micros());// us
    println!("time cost: {:?} ns", start.elapsed().as_nanos());// us
}

/*
iter.sum is: 4000002000000
time cost: 299 ms
time cost: 302236 us
time cost: 302829500 ns
*/

~~~