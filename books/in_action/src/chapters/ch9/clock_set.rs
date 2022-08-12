
#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;
use std::mem::zeroed;

use clap::{App, Arg};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    // Makes OS-specific imports within the function to avoid polluting the global scope.
    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::TimeZone;
        use libc::{settimeofday, timezone};
        use libc::{suseconds_t, time_t, timeval};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }

    // win 平台下等价libc 的是kernel32.dll
    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::WeekDay;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            WeekDay::Mon => 1,
            WeekDay::Tue => 2,
            WeekDay::Wed => 3,
            WeekDay::Thu => 4,
            WeekDay::Fri => 5,
            WeekDay::Sat => 6,
            WeekDay::Sun => 0,
        };

        let mut ns = t.nanosecond();

        let mut leap = 0;
        let is_leap_second = ns > 1_000_000_000;
        if is_leap_second {
            ns -= 1_000_000_000;
            leap += 1;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = (leap + t.second()) as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }
}

pub fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets and (aspirationally) sets the time.)")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                // .short("s")
                .short('s')
                .long("standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(Arg::with_name("datetime").help(
            "When <action> is 'set', apply <datetime>. \
            Otherwise, ignore.",
        ));

    let mut args = std::env::args_os();
    args.next();
    println!("{:?}", args);
    // let args = app.get_matches();
    let args = app.get_matches_from(&mut args);

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std{
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        
        };

        let err_msg = format!("Unable to pase {} according to {}",t_, std);
        let t = parser(t_).expect(&err_msg);
        Clock::set(t);
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
