#[derive(Debug, PartialEq)]
pub struct Clock {
    total_minutes: i16,
}

const DAY_MINS: i16 = 60 * 24;

impl Clock {
    pub fn minutes(&self) -> i16 {
	self.total_minutes % 60
    }

    pub fn hours(&self) -> i16 {
	self.total_minutes / 60
    }

    pub fn from_minutes(minutes: i16) -> Self {
        Clock {
	    total_minutes: minutes.rem_euclid(DAY_MINS)
        }
    }

    pub fn new(hour: i16, minute: i16) -> Self {
        Clock::from_minutes(hour * 60 + minute)
    }

    pub fn add_minutes(&self, minutes: i16) -> Self {
        Clock::from_minutes(self.total_minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
