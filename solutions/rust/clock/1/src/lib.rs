use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clock {
    inner: DayMinute,
}

// Represents the minute in a day.
// Can only be 0 to 1339.
// Ex.: 0 is 00:00, 1339 is 23:59
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
struct DayMinute(u16);

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hour = self.hours();
        let remainder = self.minutes();

        write!(f, "{hour:02}:{remainder:02}")
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            inner: DayMinute::new(hours, minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            inner: self.inner.add(minutes),
        }
    }

    pub fn hours(&self) -> u8 {
        self.inner.hours()
    }

    pub fn minutes(&self) -> u8 {
        self.inner.minutes()
    }
}

impl DayMinute {
    const MAX: i32 = 24 * 60;

    fn new(hours: i32, minutes: i32) -> Self {
        Self::from(hours * 60 + minutes)
    }

    fn add(&self, minutes: i32) -> Self {
        let sum = self.0 as i32 + minutes;

        Self::from(sum)
    }

    fn from(unormalized: i32) -> Self {
        let normalized = normalize(unormalized);

        Self(normalized)
    }

    fn hours(&self) -> u8 {
        (self.0 / 60) as u8
    }

    fn minutes(&self) -> u8 {
        (self.0 - self.hours() as u16 * 60_u16) as u8
    }
}

fn normalize(unormalized: i32) -> u16 {
    if (0..DayMinute::MAX).contains(&unormalized) {
        return unormalized as u16;
    }

    match unormalized {
        n if n >= DayMinute::MAX => normalize(unormalized.abs() - DayMinute::MAX),
        n if n < 0 => normalize(DayMinute::MAX - unormalized.abs()),
        _ => unormalized as u16,
    }
}
