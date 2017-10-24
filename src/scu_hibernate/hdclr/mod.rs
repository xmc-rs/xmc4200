#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HDCLR {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = "Values that can be written to the field `EPEV`"]
pub enum EPEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear wake-up event"]
    VALUE2,
}
impl EPEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPEVW::VALUE1 => false,
            EPEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPEVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPEVW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPEVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENEV`"]
pub enum ENEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear wake-up event"]
    VALUE2,
}
impl ENEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENEVW::VALUE1 => false,
            ENEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENEVW<'a> {
    w: &'a mut W,
}
impl<'a> _ENEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENEVW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENEVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCEV`"]
pub enum RTCEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear wake-up event"]
    VALUE2,
}
impl RTCEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCEVW::VALUE1 => false,
            RTCEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCEVW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCEVW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCEVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ULPWDG`"]
pub enum ULPWDGW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear watchdog alarm"]
    VALUE2,
}
impl ULPWDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPWDGW::VALUE1 => false,
            ULPWDGW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPWDGW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPWDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPWDGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDGW::VALUE1)
    }
    #[doc = "Clear watchdog alarm"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDGW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VBATPEV`"]
pub enum VBATPEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear wake-up event"]
    VALUE2,
}
impl VBATPEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBATPEVW::VALUE1 => false,
            VBATPEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBATPEVW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATPEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBATPEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATPEVW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATPEVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VBATNEV`"]
pub enum VBATNEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear wake-up event"]
    VALUE2,
}
impl VBATNEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBATNEVW::VALUE1 => false,
            VBATNEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBATNEVW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATNEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBATNEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATNEVW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATNEVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHIBIO0PEV`"]
pub enum AHIBIO0PEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear wake-up event"]
    VALUE2,
}
impl AHIBIO0PEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHIBIO0PEVW::VALUE1 => false,
            AHIBIO0PEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHIBIO0PEVW<'a> {
    w: &'a mut W,
}
impl<'a> _AHIBIO0PEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHIBIO0PEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0PEVW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0PEVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHIBIO0NEV`"]
pub enum AHIBIO0NEVW {
    #[doc = "No effect"]
    VALUE1,
    #[doc = "Clear wake-up event"]
    VALUE2,
}
impl AHIBIO0NEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHIBIO0NEVW::VALUE1 => false,
            AHIBIO0NEVW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHIBIO0NEVW<'a> {
    w: &'a mut W,
}
impl<'a> _AHIBIO0NEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHIBIO0NEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0NEVW::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0NEVW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Clear"]
    #[inline]
    pub fn epev(&mut self) -> _EPEVW {
        _EPEVW { w: self }
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Clear"]
    #[inline]
    pub fn enev(&mut self) -> _ENEVW {
        _ENEVW { w: self }
    }
    #[doc = "Bit 2 - RTC Event Clear"]
    #[inline]
    pub fn rtcev(&mut self) -> _RTCEVW {
        _RTCEVW { w: self }
    }
    #[doc = "Bit 3 - ULP WDG Alarm Clear"]
    #[inline]
    pub fn ulpwdg(&mut self) -> _ULPWDGW {
        _ULPWDGW { w: self }
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear"]
    #[inline]
    pub fn vbatpev(&mut self) -> _VBATPEVW {
        _VBATPEVW { w: self }
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear"]
    #[inline]
    pub fn vbatnev(&mut self) -> _VBATNEVW {
        _VBATNEVW { w: self }
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline]
    pub fn ahibio0pev(&mut self) -> _AHIBIO0PEVW {
        _AHIBIO0PEVW { w: self }
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline]
    pub fn ahibio0nev(&mut self) -> _AHIBIO0NEVW {
        _AHIBIO0NEVW { w: self }
    }
}
