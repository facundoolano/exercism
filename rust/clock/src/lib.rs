use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: normalized_hours(hours, minutes),
            minutes: modulo(minutes, 60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn normalized_hours(hours: i32, minutes: i32) -> i32 {
    let hour_diff = if minutes < 0 && minutes % 60 != 0 {
        -1 + minutes / 60
    } else {
        minutes / 60
    };
    modulo(hours + hour_diff, 24)
}

fn modulo(n: i32, div: i32) -> i32 {
    ((n % div) + div) % div
}
