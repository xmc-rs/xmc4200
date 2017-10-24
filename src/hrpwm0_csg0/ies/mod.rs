#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IES {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SVLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVLSR {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active when input is HIGH"]
    VALUE2,
    #[doc = "Active when input is LOW"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SVLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SVLSR::VALUE1 => 0,
            SVLSR::VALUE2 => 1,
            SVLSR::VALUE3 => 2,
            SVLSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SVLSR {
        match value {
            0 => SVLSR::VALUE1,
            1 => SVLSR::VALUE2,
            2 => SVLSR::VALUE3,
            i => SVLSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SVLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SVLSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SVLSR::VALUE3
    }
}
#[doc = "Possible values of the field `STRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRESR {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl STRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STRESR::VALUE1 => 0,
            STRESR::VALUE2 => 1,
            STRESR::VALUE3 => 2,
            STRESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STRESR {
        match value {
            0 => STRESR::VALUE1,
            1 => STRESR::VALUE2,
            2 => STRESR::VALUE3,
            3 => STRESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STRESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STRESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STRESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STRESR::VALUE4
    }
}
#[doc = "Possible values of the field `STPES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPESR {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl STPESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STPESR::VALUE1 => 0,
            STPESR::VALUE2 => 1,
            STPESR::VALUE3 => 2,
            STPESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STPESR {
        match value {
            0 => STPESR::VALUE1,
            1 => STPESR::VALUE2,
            2 => STPESR::VALUE3,
            3 => STPESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STPESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STPESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STPESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STPESR::VALUE4
    }
}
#[doc = "Possible values of the field `TRGES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGESR {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl TRGESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGESR::VALUE1 => 0,
            TRGESR::VALUE2 => 1,
            TRGESR::VALUE3 => 2,
            TRGESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGESR {
        match value {
            0 => TRGESR::VALUE1,
            1 => TRGESR::VALUE2,
            2 => TRGESR::VALUE3,
            3 => TRGESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRGESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRGESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRGESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TRGESR::VALUE4
    }
}
#[doc = "Possible values of the field `STES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STESR {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl STESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STESR::VALUE1 => 0,
            STESR::VALUE2 => 1,
            STESR::VALUE3 => 2,
            STESR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STESR {
        match value {
            0 => STESR::VALUE1,
            1 => STESR::VALUE2,
            2 => STESR::VALUE3,
            3 => STESR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STESR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STESR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STESR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STESR::VALUE4
    }
}
#[doc = "Values that can be written to the field `SVLS`"]
pub enum SVLSW {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active when input is HIGH"]
    VALUE2,
    #[doc = "Active when input is LOW"]
    VALUE3,
}
impl SVLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SVLSW::VALUE1 => 0,
            SVLSW::VALUE2 => 1,
            SVLSW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVLSW<'a> {
    w: &'a mut W,
}
impl<'a> _SVLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVLSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Function disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVLSW::VALUE1)
    }
    #[doc = "Active when input is HIGH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVLSW::VALUE2)
    }
    #[doc = "Active when input is LOW"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SVLSW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STRES`"]
pub enum STRESW {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl STRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STRESW::VALUE1 => 0,
            STRESW::VALUE2 => 1,
            STRESW::VALUE3 => 2,
            STRESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STRESW<'a> {
    w: &'a mut W,
}
impl<'a> _STRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STRESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Function disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRESW::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRESW::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STRESW::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STRESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STPES`"]
pub enum STPESW {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl STPESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STPESW::VALUE1 => 0,
            STPESW::VALUE2 => 1,
            STPESW::VALUE3 => 2,
            STPESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPESW<'a> {
    w: &'a mut W,
}
impl<'a> _STPESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Function disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STPESW::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STPESW::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STPESW::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STPESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGES`"]
pub enum TRGESW {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl TRGESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGESW::VALUE1 => 0,
            TRGESW::VALUE2 => 1,
            TRGESW::VALUE3 => 2,
            TRGESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGESW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Function disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRGESW::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRGESW::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRGESW::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRGESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STES`"]
pub enum STESW {
    #[doc = "Function disabled"]
    VALUE1,
    #[doc = "Active on rising edge"]
    VALUE2,
    #[doc = "Active on falling edge"]
    VALUE3,
    #[doc = "Active on both edges"]
    VALUE4,
}
impl STESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STESW::VALUE1 => 0,
            STESW::VALUE2 => 1,
            STESW::VALUE3 => 2,
            STESW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STESW<'a> {
    w: &'a mut W,
}
impl<'a> _STESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Function disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STESW::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STESW::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STESW::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STESW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline]
    pub fn svls(&self) -> SVLSR {
        SVLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline]
    pub fn stres(&self) -> STRESR {
        STRESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline]
    pub fn stpes(&self) -> STPESR {
        STPESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline]
    pub fn trges(&self) -> TRGESR {
        TRGESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline]
    pub fn stes(&self) -> STESR {
        STESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline]
    pub fn svls(&mut self) -> _SVLSW {
        _SVLSW { w: self }
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline]
    pub fn stres(&mut self) -> _STRESW {
        _STRESW { w: self }
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline]
    pub fn stpes(&mut self) -> _STPESW {
        _STPESW { w: self }
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline]
    pub fn trges(&mut self) -> _TRGESW {
        _TRGESW { w: self }
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline]
    pub fn stes(&mut self) -> _STESW {
        _STESW { w: self }
    }
}
