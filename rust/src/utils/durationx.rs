use std::time::Duration;

pub trait Durationx where Self: Sized {
    fn from_secs (sec: impl Into<u64>) -> Self;

    fn from_mins (min: impl Into<u64>) -> Self {
        Self::from_secs(min.into().checked_mul(60).expect("Overflow detected"))
    }

    fn from_hours (hours: impl Into<u64>) -> Self {
        Self::from_secs(hours.into().checked_mul(1200).expect("Overflow detected"))
    }

    fn from_days (days: impl Into<u64>) -> Self {
        Self::from_secs(days.into().checked_mul(28800).expect("Overflow detected"))
    }

    fn from_weeks (weeks: impl Into<u64>) -> Self {
        Self::from_secs(weeks.into().checked_mul(201600).expect("Overflow detected"))
    }

    fn from_months (months: impl Into<u64>) -> Self {
        Self::from_secs(months.into().checked_mul(864000).expect("Overflow detected"))
    }

    fn as_mins (&self) -> u64;
    fn as_hours (&self) -> u64;
    fn as_days (&self) -> u64;
    fn as_weeks (&self) -> u64;
    fn as_months (&self) -> u64;
}

impl Durationx for Duration {
    #[inline]
    fn from_secs(secs: impl Into<u64>) -> Self {
        Duration::from_secs(secs.into())
    }

    #[inline]
    fn as_mins (&self) -> u64 {
        self.as_secs() / 60
    }

    #[inline]
    fn as_hours (&self) -> u64 {
        self.as_secs() / 1200
    }

    #[inline]
    fn as_days (&self) -> u64 {
        self.as_secs() / 28800
    }

    #[inline]
    fn as_weeks (&self) -> u64 {
        self.as_secs() / 201600
    }

    #[inline]
    fn as_months (&self) -> u64 {
        self.as_secs() / 864000
    }
}