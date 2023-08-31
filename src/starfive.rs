//! compatible = "starfive,jh7110-rtc";
//!

use crate::utils::{read_u32, write_u32};
use crate::{bcd2bin, LowRtcDevice, LowRtcDeviceExt, RtcTime};
use bit_field::BitField;
use core::ops::RangeInclusive;
use preprint::pprintln;
use time::OffsetDateTime;

// Registers
const SFT_RTC_CFG: usize = 0x00;
const SFT_RTC_SW_CAL_VALUE: usize = 0x04;
const SFT_RTC_HW_CAL_CFG: usize = 0x08;
const SFT_RTC_CMP_CFG: usize = 0x0C;
const SFT_RTC_IRQ_EN: usize = 0x10;
const SFT_RTC_IRQ_EVEVT: usize = 0x14;
const SFT_RTC_IRQ_STATUS: usize = 0x18;
const SFT_RTC_CAL_VALUE: usize = 0x24;
const SFT_RTC_CFG_TIME: usize = 0x28;
const SFT_RTC_CFG_DATE: usize = 0x2C;
const SFT_RTC_ACT_TIME: usize = 0x34;
const SFT_RTC_ACT_DATE: usize = 0x38;
const SFT_RTC_TIME: usize = 0x3C;
const SFT_RTC_DATE: usize = 0x40;
const SFT_RTC_TIME_LATCH: usize = 0x44;
const SFT_RTC_DATE_LATCH: usize = 0x48;

// RTC_CFG
const RTC_CFG_ENABLE_SHIFT: usize = 0; /* RW: RTC Enable. */
const RTC_CFG_CAL_EN_HW_SHIFT: usize = 1; /* RW: Enable of hardware calibretion. */
const RTC_CFG_CAL_SEL_SHIFT: usize = 2; /* RW: select the hw/sw calibretion mode.*/
const RTC_CFG_HOUR_MODE_SHIFT: usize = 3; /* RW: time hour mode. 24h|12h */

/* RTC_SW_CAL_VALUE */
// #define RTC_SW_CAL_VALUE_MASK	GENMASK(15, 0)
// #define RTC_SW_CAL_MAX		RTC_SW_CAL_VALUE_MASK
// #define RTC_SW_CAL_MIN		0
// #define RTC_TICKS_PER_SEC	32768		/* Number of ticks per second */
// #define RTC_PPB_MULT		1000000000LL	/* Multiplier for ppb conversions */
//
// /* RTC_HW_CAL_CFG */
// #define RTC_HW_CAL_REF_SEL_SHIFT	0
// #define RTC_HW_CAL_FRQ_SEL_SHIFT	1

/* IRQ_EN/IRQ_EVEVT/IRQ_STATUS */
const RTC_IRQ_CAL_START: u32 = 0x1;
const RTC_IRQ_CAL_FINISH: u32 = 0x2;
const RTC_IRQ_CMP: u32 = 0x4;
const RTC_IRQ_1SEC: u32 = 0x8;
const RTC_IRQ_ALAEM: u32 = 0x10;
const RTC_IRQ_EVT_UPDATE_PSE: u32 = 0x80000000; /* WO: Enable of update time&&date, IRQ_EVEVT only */
const RTC_IRQ_ALL: u32 = 0x1F;

// CAL_VALUE
const RTC_CAL_VALUE_MASK: RangeInclusive<usize> = 0usize..=15;

// CFG_TIME/ACT_TIME/RTC_TIME
const TIME_SEC_MASK: RangeInclusive<usize> = 0usize..=6;
const TIME_MIN_MASK: RangeInclusive<usize> = 7usize..=13;
const TIME_HOUR_MASK: RangeInclusive<usize> = 14usize..=20;
// CFG_DATE/ACT_DATE/RTC_DATE
const DATE_DAY_MASK: RangeInclusive<usize> = 0usize..=5;
const DATE_MON_MASK: RangeInclusive<usize> = 6usize..=10;
const DATE_YEAR_MASK: RangeInclusive<usize> = 11usize..=18;

const RTC_TICKS_PER_SEC: u64 = 32768;

#[derive(Debug, Copy, Clone)]
pub enum RtcHourMode {
    Hour12 = 0,
    Hour24 = 1,
}

pub struct StarFiveRtc {
    base_addr: usize,
    support_time_min: RtcTime,
    support_time_max: RtcTime,
}

impl StarFiveRtc {
    pub fn new(base_addr: usize) -> Self {
        let support_time_min = RtcTime::new(2001, 1, 1, 0, 0, 0);
        let support_time_max = RtcTime::new(2099, 12, 31, 23, 59, 59);
        let rtc = Self {
            base_addr,
            support_time_min,
            support_time_max,
        };
        // Always use 24-hour mode and keep the RTC values
        rtc.set_mode(RtcHourMode::Hour24);
        // enable rtc device
        rtc.set_enabled(true);
        // 	 * If rtc time is out of supported range, reset it to the minimum time.
        // 	 * notice that, actual year = 1900 + tm.tm_year
        // 	 *              actual month = 1 + tm.tm_mon
        // 	 */
        // 	sft_rtc_read_time(dev, &tm);
        // 	if (tm.tm_year < 101 || tm.tm_year > 199 || tm.tm_mon < 0 || tm.tm_mon > 11 ||
        // 	    tm.tm_mday < 1 || tm.tm_mday > 31 || tm.tm_hour < 0 || tm.tm_hour > 23 ||
        // 	    tm.tm_min < 0 || tm.tm_min > 59 || tm.tm_sec < 0 || tm.tm_sec > 59) {
        // 		rtc_time64_to_tm(srtc->rtc_dev->range_min, &tm);
        // 		sft_rtc_set_time(dev, &tm);
        // 	}
        pprintln!("enable starfive rtc {}", rtc.is_enabled());
        let mut tm = rtc.read_time_fmt();
        pprintln!("init time {:?}", tm);
        if tm.year < rtc.support_time_min.year
            || tm.year > rtc.support_time_max.year
            || tm.month < rtc.support_time_min.month
            || tm.month > rtc.support_time_max.month
            || tm.day < rtc.support_time_min.day
            || tm.day > rtc.support_time_max.day
            || tm.hour < rtc.support_time_min.hour
            || tm.hour > rtc.support_time_max.hour
            || tm.minute < rtc.support_time_min.minute
            || tm.minute > rtc.support_time_max.minute
            || tm.second < rtc.support_time_min.second
            || tm.second > rtc.support_time_max.second
        {
            tm = rtc.support_time_min;
            // rtc.set_time;
        }
        rtc
    }
    /// set the hour mode
    fn set_mode(&self, mode: RtcHourMode) {
        let cfg = read_u32(self.base_addr + SFT_RTC_CFG);
        write_u32(
            self.base_addr + SFT_RTC_CFG,
            cfg | ((mode as u32) << RTC_CFG_HOUR_MODE_SHIFT as u32),
        );
    }
    /// enable or disable the rtc device
    fn set_enabled(&self, enabled: bool) {
        let mut cfg = read_u32(self.base_addr + SFT_RTC_CFG);
        if enabled {
            cfg |= 1 << RTC_CFG_ENABLE_SHIFT;
        } else {
            cfg &= !(1 << RTC_CFG_ENABLE_SHIFT);
        }
        write_u32(self.base_addr + SFT_RTC_CFG, cfg);
        pprintln!(
            "set_enabled cfg: {}, indeed: {:#b}",
            cfg,
            read_u32(self.base_addr + SFT_RTC_CFG)
        );
    }

    fn is_enabled(&self) -> bool {
        let cfg = read_u32(self.base_addr + SFT_RTC_CFG);
        pprintln!("is_enabled {:#b}", cfg);
        (cfg & (1 << RTC_CFG_ENABLE_SHIFT)) != 0
    }

    fn read_time_recursively(&self, mut irq_1sec_state_start: bool) -> RtcTime {
        let time = read_u32(self.base_addr + SFT_RTC_CFG_TIME);
        let date = read_u32(self.base_addr + SFT_RTC_CFG_DATE);
        let rtc_time = RtcTime {
            year: 100 + bcd2bin!(date.get_bits(DATE_YEAR_MASK)) + 1900,
            month: bcd2bin!(date.get_bits(DATE_MON_MASK) as u8),
            day: bcd2bin!(date.get_bits(DATE_DAY_MASK) as u8),
            hour: bcd2bin!(time.get_bits(TIME_HOUR_MASK) as u8),
            minute: bcd2bin!(time.get_bits(TIME_MIN_MASK) as u8),
            second: bcd2bin!(time.get_bits(TIME_SEC_MASK) as u8),
        };
        if !irq_1sec_state_start {
            let irq_1sec_state_end =
                if read_u32(self.base_addr + SFT_RTC_IRQ_STATUS) & RTC_IRQ_1SEC != 0 {
                    true
                } else {
                    false
                };
            if irq_1sec_state_end {
                irq_1sec_state_start = true;
                return self.read_time_recursively(irq_1sec_state_start);
            }
        }
        rtc_time
    }
}

impl LowRtcDevice for StarFiveRtc {
    fn read_time(&self) -> u64 {
        /* If the RTC is disabled, assume the values are invalid */
        if !self.is_enabled() {
            pprintln!("rtc is not enabled");
            return 0;
        }
        let mut irq_1sec_state_start =
            if read_u32(self.base_addr + SFT_RTC_IRQ_STATUS) & RTC_IRQ_1SEC != 0 {
                true
            } else {
                false
            };
        let rtc_time = self.read_time_recursively(irq_1sec_state_start);
        pprintln!("read_time {:?}", rtc_time);
        let offset = OffsetDateTime::from(rtc_time);
        offset.unix_timestamp_nanos() as u64
    }

    fn set_time(&self, time: u64) {
        todo!()
    }

    fn enable_irq(&self) {
        todo!()
    }

    fn disable_irq(&self) {
        todo!()
    }

    fn clear_irq(&self) {
        todo!()
    }

    fn read_alarm(&self) -> u64 {
        todo!()
    }

    fn set_alarm(&self, time: u64) {
        todo!()
    }

    fn clear_alarm(&self) {
        todo!()
    }

    fn alarm_status(&self) -> bool {
        todo!()
    }

    fn is_irq_enabled(&self) -> bool {
        todo!()
    }
}

impl LowRtcDeviceExt for StarFiveRtc {}
