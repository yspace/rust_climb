use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn it_() {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("{}", time);

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Current Micro Seconds: {}", time.as_micros());
    println!("Current Nano Seconds:  {}", time.as_nanos());
    println!("Current Seconds: {}", time.as_secs());
    println!("Current Seconds in f64: {}", time.as_secs_f64());
}
// @see https://rustwiki.org/zh-CN/rust-cookbook/datetime/parse.html

mod _1 {
    use chrono::{Datelike, Timelike, Utc};

    #[test]
    fn main() {
        let now = Utc::now();

        let (is_pm, hour) = now.hour12();
        println!(
            "The current UTC time is {:02}:{:02}:{:02} {}",
            hour,
            now.minute(),
            now.second(),
            if is_pm { "PM" } else { "AM" }
        );
        println!(
            "And there have been {} seconds since midnight",
            now.num_seconds_from_midnight()
        );

        let (is_common_era, year) = now.year_ce();
        println!(
            "The current UTC date is {}-{:02}-{:02} {:?} ({})",
            year,
            now.month(),
            now.day(),
            now.weekday(),
            if is_common_era { "CE" } else { "BCE" }
        );
        println!(
            "And the Common Era began {} days ago",
            now.num_days_from_ce()
        );
    }
}

mod _2 {
    use chrono::{NaiveDate, NaiveDateTime};

    fn main() {
        let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
        println!(
            "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
            date_time,
            date_time.timestamp()
        );

        let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
        println!(
            "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
            date_time_after_a_billion_seconds
        );
    }
}

mod _3 {
    use chrono::{DateTime, Utc};

    fn main() {
        let now: DateTime<Utc> = Utc::now();

        println!("UTC now is: {}", now);
        println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
        println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
        println!(
            "UTC now in a custom format is: {}",
            now.format("%a %b %e %T %Y")
        );
    }
}

mod _4 {
    use chrono::format::ParseError;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

    fn main() -> Result<(), ParseError> {
        let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
        println!("{}", rfc2822);

        let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
        println!("{}", rfc3339);

        let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
        println!("{}", custom);

        let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
        println!("{}", time_only);

        let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
        println!("{}", date_only);

        let no_timezone =
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
        println!("{}", no_timezone);

        Ok(())
    }
}
