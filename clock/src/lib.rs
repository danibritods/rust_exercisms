use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Clock {hours, minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // unimplemented!("Add {minutes} minutes to existing Clock time");
        let total_minutes = self.minutes + minutes;
        let total_hours = self.hours + (total_minutes / 60);
        let new_minutes = total_minutes % 60;
        let new_hours = total_hours % 24;

        Clock {
            hours: new_hours,
            minutes: new_minutes
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}