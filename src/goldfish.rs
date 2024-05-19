//! The driver for the Goldfish RTC device.
//!
//! compatible = "googlRtcDevicee,goldfish-rtc";

use crate::{LowRtcDevice, RtcIORegion};
use alloc::boxed::Box;
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
    region: Box<dyn RtcIORegion>,
}

impl GoldFishRtc {
    pub fn new(region: Box<dyn RtcIORegion>) -> Self {
        let rtc = Self { region };
        rtc
    }
}

impl LowRtcDevice for GoldFishRtc {
    fn read_time(&self) -> u64 {
        let time_low = self.region.read_at(RTC_TIME_LOW);
        let time_high = self.region.read_at(RTC_TIME_HIGH);
        (time_high as u64) << 32 | time_low as u64
    }

    fn set_time(&self, time: u64) {
        let time_low = time as u32;
        let time_high = (time >> 32) as u32;
        self.region.write_at(RTC_TIME_LOW, time_low);
        self.region.write_at(RTC_TIME_HIGH, time_high);
    }

    fn enable_irq(&self) {
        self.region.write_at(RTC_IRQ_ENABLED, 1);
    }

    fn disable_irq(&self) {
        self.region.write_at(RTC_IRQ_ENABLED, 0);
    }

    fn clear_irq(&self) {
        self.region.write_at(RTC_CLEAR_INTERRUPT, 1);
    }

    fn read_alarm(&self) -> u64 {
        let alarm_low = self.region.read_at(RTC_ALARM_LOW);
        let alarm_high = self.region.read_at(RTC_ALARM_HIGH);
        (alarm_high as u64) << 32 | alarm_low as u64
    }

    fn set_alarm(&self, time: u64) {
        let alarm_low = time as u32;
        let alarm_high = (time >> 32) as u32;
        self.region.write_at(RTC_ALARM_LOW, alarm_low);
        self.region.write_at(RTC_ALARM_HIGH, alarm_high);
    }

    fn clear_alarm(&self) {
        self.region.write_at(RTC_CLEAR_ALARM, 1)
    }

    fn alarm_status(&self) -> bool {
        let alarm_status = self.region.read_at(RTC_ALARM_STATUS);
        alarm_status == 1
    }

    fn is_irq_enabled(&self) -> bool {
        let irq_enabled = self.region.read_at(RTC_IRQ_ENABLED);
        irq_enabled == 1
    }
}
