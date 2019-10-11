use std::fmt;

pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock(self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hour = ((self.0 / 60) as f32).floor();
        let minute = self.0 % 60;
        write!(f, "{}:{}", hour, minute)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hour = ((self.0 / 60) as f32).floor();
        let minute = self.0 % 60;
        write!(f, "{}:{}", hour, minute)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl std::cmp::Eq for Clock {
}
