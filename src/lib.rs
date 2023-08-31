#![cfg_attr(not(test), no_std)]
#![feature(type_name_of_val)]
#![allow(unused)]
#[cfg(feature = "goldfish")]
pub mod goldfish;
#[cfg(feature = "starfive")]
pub mod starfive;
mod utils;

extern crate alloc;

use ::time::macros::offset;
use ::time::{Date, Month, OffsetDateTime, Time};
use core::fmt::{Debug, Formatter};

pub trait LowRtcDevice {
    /// read time in nanoseconds
    fn read_time(&self) -> u64;
    /// set time in nanoseconds
    fn set_time(&self, time: u64);
    /// enable interrupt
    fn enable_irq(&self);
    /// disable interrupt
    fn disable_irq(&self);
    /// clear interrupt
    fn clear_irq(&self);
    /// read alarm in nanoseconds
    fn read_alarm(&self) -> u64;
    /// set alarm in nanoseconds
    fn set_alarm(&self, time: u64);
    /// clear alarm
    fn clear_alarm(&self);
    fn alarm_status(&self) -> bool;
    fn is_irq_enabled(&self) -> bool;
}

pub trait LowRtcDeviceExt: LowRtcDevice {
    fn read_time_fmt(&self) -> RtcTime {
        let time_stamp = self.read_time();
        let t =
            OffsetDateTime::from_unix_timestamp_nanos(time_stamp as i128).expect("invalid time");
        let t = t.to_offset(offset!(+8));
        RtcTime::from(t)
    }

    fn read_alarm_fmt(&self) -> RtcTime {
        let time_stamp = self.read_alarm();
        let t =
            OffsetDateTime::from_unix_timestamp_nanos(time_stamp as i128).expect("invalid time");
        let t = t.to_offset(offset!(+8));
        RtcTime::from(t)
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl RtcTime {
    pub fn new(year: u32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl Debug for RtcTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}:{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl From<OffsetDateTime> for RtcTime {
    fn from(value: OffsetDateTime) -> Self {
        Self {
            year: value.year() as u32,
            month: value.month() as u8,
            day: value.day(),
            hour: value.hour(),
            minute: value.minute(),
            second: value.second(),
        }
    }
}

impl From<RtcTime> for OffsetDateTime {
    fn from(value: RtcTime) -> Self {
        let date = Date::from_calendar_date(
            value.year as i32,
            Month::try_from(value.month).unwrap(),
            value.day as u8,
        )
        .expect("invalid date");
        let time = Time::from_hms(value.hour, value.minute, value.second).expect("invalid time");
        let t = OffsetDateTime::from_unix_timestamp_nanos(0).expect("invalid time");
        let t = t.replace_date(date).replace_time(time);
        t
    }
}
