use std::{thread::sleep, time::Duration};
use time::*;

pub fn run() {
    use time::OffsetDateTime;

    let now = OffsetDateTime::now_utc();
    // let local = OffsetDateTime::now_local();

    println!("{now}");
    {
        use time::macros::datetime;
        use time::Duration;

        let a = datetime!(2022-01-01 10:00:55);
        let b = datetime!(2022-01-01 13:00:00);

        let duration: Duration = b - a;

        println!("{}", b - a);
        // 2h59m5s
    }

    let start = time::Instant::now(); //获取开始时间
    sleep(Duration::from_secs(6));
    let end = time::Instant::now();
    println!("睡了 {} 长时间", end - start);
}

fn run2() {
    use time::Weekday;
    let weekday = Weekday::Friday;
    println!("{}", weekday);

    use time::Month;
    let mut month = Month::May;
    for i in 0..12 {
        month = month.next();
        println!("{}", month);
    }
}

#[test]
fn test_run() {
    // run();
    run2();
}

mod chronos {
    use std::str::FromStr;

    use chrono::Datelike;
    use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, Utc};

    #[derive(Debug)]
    struct Message {
        text: String,
        // created_at: NaiveDateTime,
        // created_at: DateTime<Local>,
        // created_at: DateTime<FixedOffset>,
        created_at: DateTime<Utc>,
    }

    #[test]
    pub fn run() {
        let utc = Utc::now(); // 世界时间
        let local = Local::now();

        println!("{}", utc.timestamp());
        // 时间增减操作
        let tomorrow_thistime = utc + Duration::days(1);
        println!(
            "可以时间计算 {}",
            tomorrow_thistime.format("%Y-%m-%d %H:%M:%S")
        );

        // 时区
        let hour = 3600;
        let datetime_with_timezone = utc.with_timezone(&FixedOffset::east_opt(8 * hour).unwrap());
        println!("{}", datetime_with_timezone);

        let time_string = utc.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("time string {}", time_string);

        let _ndt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
        let dt = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11), Utc);
        println!("datetime string: {}", dt);

        //
        let msg = Message {
            text: "Hello".to_string(),
            created_at: Utc::now(),
        };
        println!("{:?}", msg);

        //
        //打印当前日期时间
        let now: DateTime<Local> = Local::now();
        let w = [
            "星期一",
            "星期二",
            "星期三",
            "星期四",
            "星期五",
            "星期六",
            "星期日",
        ];
        println!(
            "今天是{}{} {}",
            now.format("%Y年%m月%e日"),
            w[now.weekday().num_days_from_monday() as usize],
            now.format("%T")
        );
        //打印任意日期
        let date = NaiveDate::from_ymd(2022, 2, 14);
        let (_, year) = date.year_ce();
        println!(
            "{}年{}月{}日{}",
            year,
            date.month(),
            date.day(),
            w[date.weekday().num_days_from_monday() as usize]
        );
    }
}

mod _abc {
    use std::thread;
    use time::*;
    fn main() {
        let start = time::Instant::now(); //获取开始时间
        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let mut x = 0;
                    for _ in (0..5_000_000) {
                        x += 1
                    }
                    x
                })
            })
            .collect();
        for h in handles {
            println!(
                "Thread finished with count={}",
                h.join().map_err(|_| "Could not join a thread!").unwrap()
            );
        }
        let end = time::Instant::now(); //获取结束时间
        println!(
            "done!start : {:?},end :{:?},duration:{:?}",
            start,
            end,
            end - start
        );
    }
}
