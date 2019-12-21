use std::env;
use std::fmt;

const SECONDS_PER_YEAR: i64 = 365 * 24 * 3600;
const SECONDS_PER_LEAP_YEAR: i64 = 366 * 24 * 3600;
const SECONDS_PER_DAY: i64 = 24 * 3600;
const SECONDS_PER_HOUR: i64 = 3600;
const SECONDS_PER_MINUTE: i64 = 60;

const MONTH_DAYS_LIST: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_MONTH_DAYS_LIST: [i64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

macro_rules! die {
    ($e:expr) => {
        (|| { 
            println!($e);
            std::process::exit(1);
        }) ()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        die!("Wrong number of arguments, need one.")
    }

    let timestamp: i64 = args[1].parse().unwrap_or_else(|_| die!("Invalid unix timestamp."));

    println!("{}", parse_unix(timestamp));
}

#[derive(Default, Debug)]
struct Date {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
    timestamp: i64
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let month = match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!()
        };

        write!(f, "{} {} {}, {:02}:{:02}:{:02} UTC",
               self.day, month, self.year, self.hour, self.minute, self.second)
    }
}

fn parse_unix(t: i64) -> String {
    let sign = t.signum();

    let mut date = Date { 
        year: 1970,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        timestamp: 0
    };

    if t == 0 { return format!("{}", date); }

    // YEAR
    loop {
        let secs = seconds_of_year(date.year);

        if t - date.timestamp > 0 && t - (date.timestamp + secs) < 0 {
            break;
        }

        date.timestamp += sign * secs;
        date.year += sign * 1;
    }

    // MONTH
    {
        let month_days_list = if is_leap(date.year) { &LEAP_MONTH_DAYS_LIST } else { &MONTH_DAYS_LIST };

        for secs in month_days_list.iter().map(|d| d * SECONDS_PER_DAY) {
            if date.timestamp + secs > t { break; }

            date.timestamp += secs;
            date.month += 1;
        }
    }

    // DAY
    let day = (t - date.timestamp) / SECONDS_PER_DAY;
    date.timestamp += day * SECONDS_PER_DAY;
    date.day += day;

    // HOUR
    let hour = (t - date.timestamp) / SECONDS_PER_HOUR;
    date.timestamp += hour * SECONDS_PER_HOUR;
    date.hour += hour;

    // MINUTE
    let minute = (t - date.timestamp) / SECONDS_PER_MINUTE;
    date.timestamp += minute * SECONDS_PER_MINUTE;
    date.minute += minute;

    // SECOND
    date.second = t - date.timestamp;
    date.timestamp = t;

    format!("{}", date)
}

fn seconds_of_year(year: i64) -> i64 {
    if is_leap(year) {
        SECONDS_PER_LEAP_YEAR
    } else {
        SECONDS_PER_YEAR
    }
}

fn is_leap(year: i64) -> bool {
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}
