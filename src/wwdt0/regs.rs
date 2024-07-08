#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feed(pub u32);
impl Feed {
    #[doc = "Feed value should be 0xAA followed by 0x55."]
    #[inline(always)]
    pub const fn feed(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Feed value should be 0xAA followed by 0x55."]
    #[inline(always)]
    pub fn set_feed(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Feed {
    #[inline(always)]
    fn default() -> Feed {
        Feed(0)
    }
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mod(pub u32);
impl Mod {
    #[doc = "Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub const fn wden(&self) -> super::vals::Wden {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wden::from_bits(val as u8)
    }
    #[doc = "Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub fn set_wden(&mut self, val: super::vals::Wden) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub const fn wdreset(&self) -> super::vals::Wdreset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Wdreset::from_bits(val as u8)
    }
    #[doc = "Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub fn set_wdreset(&mut self, val: super::vals::Wdreset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub const fn wdtof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn set_wdtof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub const fn wdint(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub fn set_wdint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub const fn wdprotect(&self) -> super::vals::Wdprotect {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Wdprotect::from_bits(val as u8)
    }
    #[doc = "Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn set_wdprotect(&mut self, val: super::vals::Wdprotect) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Mod {
    #[inline(always)]
    fn default() -> Mod {
        Mod(0)
    }
}
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tc(pub u32);
impl Tc {
    #[doc = "Watchdog time-out value."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Watchdog time-out value."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tc {
    #[inline(always)]
    fn default() -> Tc {
        Tc(0)
    }
}
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tv(pub u32);
impl Tv {
    #[doc = "Counter timer value."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter timer value."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tv {
    #[inline(always)]
    fn default() -> Tv {
        Tv(0)
    }
}
#[doc = "Watchdog Warning Interrupt compare value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Warnint(pub u32);
impl Warnint {
    #[doc = "Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub const fn warnint(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub fn set_warnint(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Warnint {
    #[inline(always)]
    fn default() -> Warnint {
        Warnint(0)
    }
}
#[doc = "Watchdog Window compare value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Window(pub u32);
impl Window {
    #[doc = "Watchdog window value."]
    #[inline(always)]
    pub const fn window(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Watchdog window value."]
    #[inline(always)]
    pub fn set_window(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Window {
    #[inline(always)]
    fn default() -> Window {
        Window(0)
    }
}
