
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut minutes = minutes;
        let mut hours = hours;

        while minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        hours = (minutes / 60) + hours;
        while hours >= 24 {
            hours -= 24;
        }
        while hours < 0 {
            hours += 24;
        }

        Self {
            hours,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        Self::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
