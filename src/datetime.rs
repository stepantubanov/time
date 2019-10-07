#[cfg(feature = "std")]
use crate::Sign;
use crate::{Date, Duration, Time, Weekday};
use core::cmp::Ordering;
#[cfg(feature = "std")]
use core::convert::{From, TryFrom};
use core::ops::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "std")]
use std::time::{Duration as StdDuration, SystemTime};

/// Combined date and time.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) date: Date,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) time: Time,
}

impl DateTime {
    /// Create a new `DateTime` from the provided `Date` and `Time`.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(
    ///     DateTime::new(Date::from_ymd(2019, 1, 1), Time::from_hms(0, 0, 0)),
    ///     Date::from_ymd(2019, 1, 1).midnight(),
    /// );
    /// ```
    pub const fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Create a new `DateTime` with the current date and time.
    ///
    /// ```rust
    /// # use time::DateTime;
    /// assert!(DateTime::now().year() >= 2019);
    /// ```
    ///
    /// This method is not available with `#![no_std]`.
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        SystemTime::now().into()
    }

    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(DateTime::unix_epoch(), Date::from_ymd(1970, 1, 1).midnight());
    /// ```
    pub const fn unix_epoch() -> Self {
        Self {
            date: Date {
                year: 1970,
                ordinal: 1,
            },
            time: Time {
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: 0,
            },
        }
    }

    /// Create a `DateTime` from the provided [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{Date, DateTime};
    /// assert_eq!(DateTime::from_unix_timestamp(0), DateTime::unix_epoch());
    /// assert_eq!(
    ///     DateTime::from_unix_timestamp(1_546_300_800),
    ///     Date::from_ymd(2019, 1, 1).midnight(),
    /// );
    /// ```
    pub fn from_unix_timestamp(timestamp: i64) -> Self {
        Self::unix_epoch() + Duration::seconds(timestamp)
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time)
    /// representing the `DateTime`.
    ///
    /// ```rust
    /// # use time::{Date, DateTime};
    /// assert_eq!(DateTime::unix_epoch().timestamp(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().timestamp(), 1_546_300_800);
    /// ```
    pub fn timestamp(self) -> i64 {
        (self - Self::unix_epoch()).whole_seconds()
    }

    /// Get the `Date` component of the `DateTime`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().date(), Date::from_ymd(2019, 1, 1));
    /// ```
    pub const fn date(self) -> Date {
        self.date
    }

    /// Get the `Time` component of the `DateTime`.
    ///
    /// ```rust
    /// # use time::{Date, Time};
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().time(), Time::midnight());
    pub const fn time(self) -> Time {
        self.time
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().year(), 2019);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().year(), 2019);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).midnight().year(), 2020);
    /// ```
    pub fn year(self) -> i32 {
        self.date().year()
    }

    /// Get the month of the date. If fetching both the month and day, use
    /// [`DateTime::month_day`](DateTime::month_day) instead.
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().month(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().month(), 12);
    /// ```
    pub fn month(self) -> u8 {
        self.date().month()
    }

    /// Get the day of the date. If fetching both the month and day, use
    /// [`DateTime::month_day`](DateTime::month_day) instead.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().day(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().day(), 31);
    /// ```
    pub fn day(self) -> u8 {
        self.date().day()
    }

    /// Get the month and day of the date.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().month_day(), (1, 1));
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().month_day(), (12, 31));
    /// ```
    pub fn month_day(self) -> (u8, u8) {
        self.date().month_day()
    }

    /// Get the day of the year of the date.
    ///
    /// The returned value will always be in the range `1..=366`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().ordinal(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().ordinal(), 365);
    /// ```
    pub fn ordinal(self) -> u16 {
        self.date().ordinal()
    }

    /// Get the ISO week number of the date.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().week(), 1);
    /// assert_eq!(Date::from_ymd(2019, 10, 4).midnight().week(), 40);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).midnight().week(), 1);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).midnight().week(), 53);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).midnight().week(), 53);
    /// ```
    pub fn week(self) -> u8 {
        self.date().week()
    }

    /// Get the weekday of the date.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 2, 1).midnight().weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 3, 1).midnight().weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 4, 1).midnight().weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 5, 1).midnight().weekday(), Wednesday);
    /// assert_eq!(Date::from_ymd(2019, 6, 1).midnight().weekday(), Saturday);
    /// assert_eq!(Date::from_ymd(2019, 7, 1).midnight().weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 8, 1).midnight().weekday(), Thursday);
    /// assert_eq!(Date::from_ymd(2019, 9, 1).midnight().weekday(), Sunday);
    /// assert_eq!(Date::from_ymd(2019, 10, 1).midnight().weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 11, 1).midnight().weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 12, 1).midnight().weekday(), Sunday);
    /// ```
    pub fn weekday(self) -> Weekday {
        self.date().weekday()
    }

    /// Returns the clock hour.
    ///
    /// The returned value will always be in the range `0..=23`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(0, 0, 0).hour(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(23, 59, 59).hour(), 23);
    /// ```
    pub const fn hour(self) -> u8 {
        self.time().hour()
    }

    /// Returns the minute within the hour.
    ///
    /// The returned value will always be in the range `0..=60`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(0, 0, 0).minute(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(23, 59, 59).minute(), 59);
    /// ```
    pub const fn minute(self) -> u8 {
        self.time().minute()
    }

    /// Returns the second within the minute.
    ///
    /// The returned value will always be in the range `0..=60`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(0, 0, 0).second(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(23, 59, 59).second(), 59);
    /// ```
    pub const fn second(self) -> u8 {
        self.time().second()
    }

    /// Return the milliseconds within the second.
    ///
    /// The returned value will always be in the range `0..=1_000`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_milli(0, 0, 0, 0).millisecond(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_milli(23, 59, 59, 999).millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        self.time().millisecond()
    }

    /// Return the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..=1_000_000`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_micro(0, 0, 0, 0).microsecond(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_micro(23, 59, 59, 999_999).microsecond(), 999_999);
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.time().microsecond()
    }

    /// Return the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..=1_000_000_000`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_nano(0, 0, 0, 0).nanosecond(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_nano(23, 59, 59, 999_999_999).nanosecond(), 999_999_999);
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.time().nanosecond()
    }
}

impl Add<Duration> for DateTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        Self::new(self.date + duration, self.time + duration)
    }
}

impl AddAssign<Duration> for DateTime {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for DateTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl SubAssign<Duration> for DateTime {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl Sub<DateTime> for DateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.date - rhs.date) + (self.time - rhs.time)
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.date.cmp(&other.date) {
            Ordering::Equal => match self.time.hour.cmp(&other.time.hour) {
                Ordering::Equal => match self.time.minute.cmp(&other.time.minute) {
                    Ordering::Equal => match self.time.second.cmp(&other.time.second) {
                        Ordering::Equal => self.time.nanosecond.cmp(&other.time.nanosecond),
                        other => other,
                    },
                    other => other,
                },
                other => other,
            },
            other => other,
        }
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for DateTime {
    /// Convert a `SystemTime` to a `DateTime`.
    ///
    /// ```rust
    /// # use std::time::SystemTime;
    /// # use time::DateTime;
    /// assert_eq!(DateTime::from(SystemTime::UNIX_EPOCH), DateTime::unix_epoch());
    /// ```
    fn from(system_time: SystemTime) -> Self {
        let duration = match system_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Duration::from(duration),
            Err(err) => -Duration::from(err.duration()),
        };

        Self::unix_epoch() + duration
    }
}

#[cfg(feature = "std")]
#[allow(clippy::fallible_impl_from)]
impl From<DateTime> for SystemTime {
    /// Convert a `DateTime` to a `SystemTime`.
    ///
    /// ```rust
    /// # use std::time::SystemTime;
    /// # use time::DateTime;
    /// assert_eq!(SystemTime::from(DateTime::unix_epoch()), SystemTime::UNIX_EPOCH);
    /// ```
    fn from(datetime: DateTime) -> Self {
        let duration = datetime - DateTime::unix_epoch();

        // TODO Rewrite once `SystemTime + time::Duration` is implemented.
        match duration.sign() {
            Sign::Positive => Self::UNIX_EPOCH + StdDuration::try_from(duration).unwrap(),
            Sign::Negative => Self::UNIX_EPOCH + StdDuration::try_from(-duration).unwrap(),
            Sign::Zero => Self::UNIX_EPOCH,
            Sign::Unknown => unreachable!("Durations always have a known sign"),
        }
    }
}