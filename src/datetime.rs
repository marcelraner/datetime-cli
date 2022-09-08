mod leapyears;

use std::time::Duration;

use self::leapyears::{is_leap_year, leap_years};

const SECONDS_PER_YEAR: u32 = 31_556_926;
const SECONDS_PER_DAY: u32 = 86_400;
const DAYS_AGO_PER_MONTH: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

pub trait IntoDateTime {
    fn into_datetime(&self) -> DateTime;
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} {}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.nanosecond
        )
    }
}

impl IntoDateTime for Duration {
    fn into_datetime(&self) -> DateTime {
        let seconds = self.as_secs();
        let mut days_since_1970 = seconds / SECONDS_PER_DAY as u64;
        let seconds_of_day_left = seconds % SECONDS_PER_DAY as u64;

        let leap_years = leap_years((seconds / SECONDS_PER_YEAR as u64) + 1970) - 477;

        days_since_1970 -= leap_years;

        let mut days_left = days_since_1970 % 365;

        let mut month = 0;

        for index in (0..12).rev() {
            if days_left as i64 - DAYS_AGO_PER_MONTH[index] as i64 > 0 {
                days_left -= DAYS_AGO_PER_MONTH[index] as u64;
                month = index;
                break;
            }
        }

        if month > 2 && is_leap_year((seconds / SECONDS_PER_YEAR as u64) + 1970) {
            days_since_1970 -= 1;
        }

        DateTime {
            year: (1970 + days_since_1970 / 365) as u16,
            month: (1 + month) as u8,
            day: (1 + days_left) as u8,

            hour: (seconds_of_day_left / 3600) as u8,
            minute: ((seconds_of_day_left % 3600) / 60) as u8,
            second: (seconds_of_day_left % 60) as u8,
            nanosecond: self.subsec_nanos(),
        }
    }
}
