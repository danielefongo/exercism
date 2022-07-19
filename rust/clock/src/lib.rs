use std::cmp::{Eq, PartialEq};
use std::fmt::Display;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from_minutes(minutes + hours * 60)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::from_minutes(self.hours * 60 + self.minutes + minutes)
    }

    fn from_minutes(minutes: i32) -> Self {
        let mut final_mins = minutes;

        while final_mins < 0 {
            final_mins += 24 * 60
        }

        Clock {
            hours: (final_mins / 60) % 24,
            minutes: (final_mins) % 60,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{:0>2}:{:0>2}", self.hours, self.minutes);

        write!(f, "{}", output)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Clock {}
