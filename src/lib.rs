#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![allow(unused)]
#[cfg(feature = "goldfish")]
pub mod goldfish;
#[cfg(feature = "starfive")]
pub mod starfive;
mod utils;

extern crate alloc;

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

pub trait RtcIORegion: Debug + Send + Sync {
    fn read_at(&self, offset: usize) -> u32;
    fn write_at(&self, offset: usize, value: u32);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct RtcTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
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
