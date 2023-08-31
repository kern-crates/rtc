/// Write 4 bytes to the specified address
pub fn write_u32(addr: usize, val: u32) {
    let addr = addr as *mut u32;
    unsafe {
        addr.write_volatile(val);
    }
}

/// Read 4 bytes from the specified address
pub fn read_u32(addr: usize) -> u32 {
    let addr = addr as *mut u32;
    unsafe { addr.read_volatile() }
}

/// # Example
/// ```
/// use rtc::genmask_u32;
/// let val = genmask_u32!(1,0);
/// assert_eq!(val, 0x3);
/// ```
///
#[macro_export]
macro_rules! genmask_u32 {
    ($h:expr, $l:expr) => {
        ((!0u32 >> (32 - 1 - $h)) << $l) & !0u32 >> (32 - 1 - $h + $l)
    };
}
/// # Example
/// ```
/// use rtc::genmask_u64;
/// let val = genmask_u64!(63,0);
/// assert_eq!(val, 0xffffffffffffffff);
/// ```
#[macro_export]
macro_rules! genmask_u64 {
    ($h:expr, $l:expr) => {
        ((!0u64 >> (64 - 1 - $h)) << $l) & !0u64 >> (64 - 1 - $h + $l)
    };
}

#[macro_export]
macro_rules! bcd2bin {
    ($bcd:expr) => {{
        assert_eq!(core::any::type_name_of_val(&$bcd), "u8");
        (($bcd >> 4) * 10) + ($bcd & 0xf)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_genmask_u32() {
        let val = genmask_u32!(1, 0);
        assert_eq!(val, 0x3);
        let val = genmask_u32!(2, 0);
        assert_eq!(val, 0x7);
    }

    #[test]
    fn test_genmask_u64() {
        let val = genmask_u64!(1, 0);
        assert_eq!(val, 0x3);
        let val = genmask_u64!(2, 0);
        assert_eq!(val, 0x7);
        let val = genmask_u64!(63, 0);
        assert_eq!(val, 0xffffffffffffffff);
    }
    #[test]
    fn test_bcd2bin() {
        let v = 0x12u8;
        let val = bcd2bin!(v);
        assert_eq!(val, 12);
        let v = 0x23u8;
        let val = bcd2bin!(v);
        assert_eq!(val, 23);
        assert_eq!(99, bcd2bin!(0x99u8))
    }
    #[test]
    #[should_panic]
    fn test_bcd2bin_fail() {
        let _val = bcd2bin!(0x23);
    }
}
