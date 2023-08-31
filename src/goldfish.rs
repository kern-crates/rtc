//! compatible = "googlRtcDevicee,goldfish-rtc";

use crate::utils::{read_u32, write_u32};
use crate::{LowRtcDevice, LowRtcDeviceExt};
use core::fmt::Debug;

const RTC_TIME_LOW: usize = 0x00;
const RTC_TIME_HIGH: usize = 0x04;
const RTC_ALARM_LOW: usize = 0x08;
const RTC_ALARM_HIGH: usize = 0x0c;
const RTC_IRQ_ENABLED: usize = 0x10;
const RTC_CLEAR_ALARM: usize = 0x14;
const RTC_ALARM_STATUS: usize = 0x18;
const RTC_CLEAR_INTERRUPT: usize = 0x1c;

#[derive(Debug)]
pub struct GoldFishRtc {
    base_addr: usize,
}

impl GoldFishRtc {
    pub fn new(base_addr: usize) -> Self {
        let rtc = Self { base_addr };
        rtc
    }
}

impl LowRtcDevice for GoldFishRtc {
    fn read_time(&self) -> u64 {
        let time_low_addr = self.base_addr + RTC_TIME_LOW;
        let time_high_addr = self.base_addr + RTC_TIME_HIGH;
        let time_low = read_u32(time_low_addr);
        let time_high = read_u32(time_high_addr);
        (time_high as u64) << 32 | time_low as u64
    }

    fn set_time(&self, time: u64) {
        let time_low_addr = self.base_addr + RTC_TIME_LOW;
        let time_high_addr = self.base_addr + RTC_TIME_HIGH;
        let time_low = time as u32;
        let time_high = (time >> 32) as u32;
        write_u32(time_low_addr, time_low);
        write_u32(time_high_addr, time_high);
    }

    fn enable_irq(&self) {
        let irq_enabled_addr = self.base_addr + RTC_IRQ_ENABLED;
        write_u32(irq_enabled_addr, 1);
    }

    fn disable_irq(&self) {
        let irq_enabled_addr = self.base_addr + RTC_IRQ_ENABLED;
        write_u32(irq_enabled_addr, 0);
    }

    fn clear_irq(&self) {
        let clear_irq_addr = self.base_addr + RTC_CLEAR_INTERRUPT;
        write_u32(clear_irq_addr, 1);
    }

    fn read_alarm(&self) -> u64 {
        let alarm_low_addr = self.base_addr + RTC_ALARM_LOW;
        let alarm_high_addr = self.base_addr + RTC_ALARM_HIGH;
        let alarm_low = read_u32(alarm_low_addr);
        let alarm_high = read_u32(alarm_high_addr);
        (alarm_high as u64) << 32 | alarm_low as u64
    }

    fn set_alarm(&self, time: u64) {
        let alarm_low_addr = self.base_addr + RTC_ALARM_LOW;
        let alarm_high_addr = self.base_addr + RTC_ALARM_HIGH;
        let alarm_low = time as u32;
        let alarm_high = (time >> 32) as u32;
        write_u32(alarm_low_addr, alarm_low);
        write_u32(alarm_high_addr, alarm_high);
    }

    fn clear_alarm(&self) {
        let clear_alarm_addr = self.base_addr + RTC_CLEAR_ALARM;
        write_u32(clear_alarm_addr, 1);
    }

    fn alarm_status(&self) -> bool {
        let alarm_status_addr = self.base_addr + RTC_ALARM_STATUS;
        let alarm_status = read_u32(alarm_status_addr);
        alarm_status == 1
    }

    fn is_irq_enabled(&self) -> bool {
        let irq_enabled_addr = self.base_addr + RTC_IRQ_ENABLED;
        let irq_enabled = read_u32(irq_enabled_addr);
        irq_enabled == 1
    }
}
impl LowRtcDeviceExt for GoldFishRtc {}
