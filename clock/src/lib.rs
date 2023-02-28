use std::fmt;
use std::fmt::{Debug, Formatter};

const HOUR: i64 = 60;
const DAY: i64 = 24 * HOUR;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Clock { minutes: (hours * HOUR + minutes).rem_euclid(DAY) }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes.div_euclid(HOUR), self.minutes.rem_euclid(HOUR))
    }
}
