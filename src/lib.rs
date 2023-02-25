#![no_std]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::{Debug, Formatter};
use time::macros::offset;

#[derive(Debug)]
pub struct Rtc {
    base_addr: usize,
    #[allow(unused)]
    irq: u32,
}

#[derive(Copy, Clone)]
pub struct RtcTime {
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

impl ToString for RtcTime {
    fn to_string(&self) -> String {
        format!(
            "{}:{}:{}\n{}-{}-{}",
            self.hour, self.minute, self.second, self.year, self.month, self.day
        )
    }
}
const RTC_TIME_LOW: usize = 0x00;
const RTC_TIME_HIGH: usize = 0x04;
const RTC_ALARM_LOW: usize = 0x08;
const RTC_ALARM_HIGH: usize = 0x0c;
const RTC_IRQ_ENABLED: usize = 0x10;
const RTC_CLEAR_ALARM: usize = 0x14;
#[allow(unused)]
const RTC_ALARM_STATUS: usize = 0x18;
const RTC_CLEAR_INTERRUPT: usize = 0x1c;

impl Rtc {
    pub fn new(base_addr: usize, irq: u32) -> Self {
        Self { base_addr, irq }
    }
    pub fn read_time(&self) -> RtcTime {
        let ns_low = unsafe { ((self.base_addr + RTC_TIME_LOW) as *const u32).read_volatile() };
        let ns_high = unsafe { ((self.base_addr + RTC_TIME_HIGH) as *const u32).read_volatile() };
        // 将ns转换为当前时间
        let ns = (ns_high as u64) << 32 | ns_low as u64;
        let t = time::OffsetDateTime::from_unix_timestamp_nanos(ns as i128).expect("invalid time");
        let t = t.to_offset(offset!(+8));
        RtcTime {
            year: t.year() as u32,
            month: t.month() as u8,
            day: t.day(),
            hour: t.hour(),
            minute: t.minute(),
            second: t.second(),
        }
    }
    /// 开启中断
    pub fn enable_irq(&self) {
        unsafe {
            ((self.base_addr + RTC_IRQ_ENABLED) as *mut u32).write_volatile(1);
        }
    }
    pub fn disable_irq(&self) {
        unsafe {
            ((self.base_addr + RTC_IRQ_ENABLED) as *mut u32).write_volatile(0);
        }
    }
    pub fn set_alarm_with_next_s(&self, s: u64) {
        let ns_low = unsafe { ((self.base_addr + RTC_TIME_LOW) as *const u32).read_volatile() };
        let ns_high = unsafe { ((self.base_addr + RTC_TIME_HIGH) as *const u32).read_volatile() };
        // 将ns转换为当前时间
        let ns = (ns_high as u64) << 32 | ns_low as u64;
        let ns = 1e9 as u64 * s + ns;
        unsafe {
            ((self.base_addr + RTC_ALARM_LOW) as *mut u32)
                .write_volatile((ns & 0xffff_ffff) as u32);
            ((self.base_addr + RTC_ALARM_HIGH) as *mut u32).write_volatile((ns >> 32) as u32);
        }
    }
    pub fn clear_alarm(&self) {
        unsafe {
            ((self.base_addr + RTC_CLEAR_ALARM) as *mut u32).write_volatile(1);
        }
    }
    pub fn clear_interrupt(&self) {
        unsafe {
            ((self.base_addr + RTC_CLEAR_INTERRUPT) as *mut u32).write_volatile(1);
        }
    }
    pub fn handle_irq(&self) {
        self.clear_alarm();
        self.clear_interrupt();
    }
    pub fn get_timestamp(&self) -> u64 {
        let ns_low = unsafe { ((self.base_addr + RTC_TIME_LOW) as *const u32).read_volatile() };
        let ns_high = unsafe { ((self.base_addr + RTC_TIME_HIGH) as *const u32).read_volatile() };
        // 将ns转换为当前时间
        let ns = (ns_high as u64) << 32 | ns_low as u64;
        ns
    }
}
