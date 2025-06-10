use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const DAY_IN_MINUTES: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
       let minutes = (hours * 60 + minutes).rem_euclid(DAY_IN_MINUTES);
        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0,self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
