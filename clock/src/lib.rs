use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = (hours + (minutes / 60)) % 24;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }
        hours += if hours < 0 {24} else {0};

        Clock {hours, minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        let pattern = |x: i32| {if x < 10 {format!("0{}", x)} else {format!("{}", x)}};
        format!("{}:{}", pattern(self.hours), pattern(self.minutes))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }

    fn ne(&self, other: &Self) -> bool {
        self.hours != other.hours || self.minutes != other.minutes
    }
}
