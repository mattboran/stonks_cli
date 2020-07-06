use chrono::{Date, Datelike, Local, FixedOffset, Timelike};

pub fn est() -> FixedOffset {
    chrono::FixedOffset::west(5 * 3600)
}

pub fn is_market_holiday(date: Date<FixedOffset>) -> bool { 
    // todo: use tradier api here
    if date.month() == 7 && date.day() == 3 && date.year() == 2020 {
        true
    } else {
        false
    }
}

pub fn last_market_open_day() -> Date<FixedOffset> {
    let tz = est();
    let mut today = Local::today().with_timezone(&tz);
    while today.weekday().number_from_monday() > 5 {
        today.with_day(today.day() - 1);
        if is_market_holiday(today) {
            today.with_day(today.day() - 1);
        }
    }
    today
}