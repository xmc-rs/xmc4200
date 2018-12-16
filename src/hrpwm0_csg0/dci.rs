#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCI {
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
#[doc = "Possible values of the field `SVIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVISR {
    #[doc = "HRPWMx.SyIA"]
    VALUE1,
    #[doc = "HRPWMx.SyIB"]
    VALUE2,
    #[doc = "HRPWMx.SyIC"]
    VALUE3,
    #[doc = "HRPWMx.SyID"]
    VALUE4,
    #[doc = "HRPWMx.SyIE"]
    VALUE5,
    #[doc = "HRPWMx.SyIF"]
    VALUE6,
    #[doc = "HRPWMx.SyIG"]
    VALUE7,
    #[doc = "HRPWMx.SyIH"]
    VALUE8,
    #[doc = "HRPWMx.SyII"]
    VALUE9,
    #[doc = "HRPWMx.SyIJ"]
    VALUE10,
    #[doc = "HRPWMx.SyIK"]
    VALUE11,
    #[doc = "HRPWMx.SyIL"]
    VALUE12,
    #[doc = "HRPWMx.SyIM"]
    VALUE13,
    #[doc = "HRPWMx.SyIN"]
    VALUE14,
    #[doc = "HRPWMx.SyIO"]
    VALUE15,
    #[doc = "HRPWMx.SyIP"]
    VALUE16,
}
impl SVISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SVISR::VALUE1 => 0,
            SVISR::VALUE2 => 1,
            SVISR::VALUE3 => 2,
            SVISR::VALUE4 => 3,
            SVISR::VALUE5 => 4,
            SVISR::VALUE6 => 5,
            SVISR::VALUE7 => 6,
            SVISR::VALUE8 => 7,
            SVISR::VALUE9 => 8,
            SVISR::VALUE10 => 9,
            SVISR::VALUE11 => 10,
            SVISR::VALUE12 => 11,
            SVISR::VALUE13 => 12,
            SVISR::VALUE14 => 13,
            SVISR::VALUE15 => 14,
            SVISR::VALUE16 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SVISR {
        match value {
            0 => SVISR::VALUE1,
            1 => SVISR::VALUE2,
            2 => SVISR::VALUE3,
            3 => SVISR::VALUE4,
            4 => SVISR::VALUE5,
            5 => SVISR::VALUE6,
            6 => SVISR::VALUE7,
            7 => SVISR::VALUE8,
            8 => SVISR::VALUE9,
            9 => SVISR::VALUE10,
            10 => SVISR::VALUE11,
            11 => SVISR::VALUE12,
            12 => SVISR::VALUE13,
            13 => SVISR::VALUE14,
            14 => SVISR::VALUE15,
            15 => SVISR::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SVISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SVISR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SVISR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SVISR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == SVISR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == SVISR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == SVISR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == SVISR::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == SVISR::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == SVISR::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == SVISR::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == SVISR::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == SVISR::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == SVISR::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == SVISR::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == SVISR::VALUE16
    }
}
#[doc = r" Value of the field"]
pub struct STRISR {
    bits: u8,
}
impl STRISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STPISR {
    bits: u8,
}
impl STPISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRGISR {
    bits: u8,
}
impl TRGISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STISR {
    bits: u8,
}
impl STISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSR {
    #[doc = "HRPWMx.MCLK (Module clock is used)"]
    VALUE1,
    #[doc = "HRPWMx.ECLKA (External clock is used)"]
    VALUE2,
    #[doc = "HRPWMx.ECLKB (External clock is used)"]
    VALUE3,
    #[doc = "HRPWMx.ECLKC (External clock is used)"]
    VALUE4,
}
impl SCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCSR::VALUE1 => 0,
            SCSR::VALUE2 => 1,
            SCSR::VALUE3 => 2,
            SCSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCSR {
        match value {
            0 => SCSR::VALUE1,
            1 => SCSR::VALUE2,
            2 => SCSR::VALUE3,
            3 => SCSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SCSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SCSR::VALUE4
    }
}
#[doc = "Values that can be written to the field `SVIS`"]
pub enum SVISW {
    #[doc = "HRPWMx.SyIA"]
    VALUE1,
    #[doc = "HRPWMx.SyIB"]
    VALUE2,
    #[doc = "HRPWMx.SyIC"]
    VALUE3,
    #[doc = "HRPWMx.SyID"]
    VALUE4,
    #[doc = "HRPWMx.SyIE"]
    VALUE5,
    #[doc = "HRPWMx.SyIF"]
    VALUE6,
    #[doc = "HRPWMx.SyIG"]
    VALUE7,
    #[doc = "HRPWMx.SyIH"]
    VALUE8,
    #[doc = "HRPWMx.SyII"]
    VALUE9,
    #[doc = "HRPWMx.SyIJ"]
    VALUE10,
    #[doc = "HRPWMx.SyIK"]
    VALUE11,
    #[doc = "HRPWMx.SyIL"]
    VALUE12,
    #[doc = "HRPWMx.SyIM"]
    VALUE13,
    #[doc = "HRPWMx.SyIN"]
    VALUE14,
    #[doc = "HRPWMx.SyIO"]
    VALUE15,
    #[doc = "HRPWMx.SyIP"]
    VALUE16,
}
impl SVISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SVISW::VALUE1 => 0,
            SVISW::VALUE2 => 1,
            SVISW::VALUE3 => 2,
            SVISW::VALUE4 => 3,
            SVISW::VALUE5 => 4,
            SVISW::VALUE6 => 5,
            SVISW::VALUE7 => 6,
            SVISW::VALUE8 => 7,
            SVISW::VALUE9 => 8,
            SVISW::VALUE10 => 9,
            SVISW::VALUE11 => 10,
            SVISW::VALUE12 => 11,
            SVISW::VALUE13 => 12,
            SVISW::VALUE14 => 13,
            SVISW::VALUE15 => 14,
            SVISW::VALUE16 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVISW<'a> {
    w: &'a mut W,
}
impl<'a> _SVISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HRPWMx.SyIA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVISW::VALUE1)
    }
    #[doc = "HRPWMx.SyIB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVISW::VALUE2)
    }
    #[doc = "HRPWMx.SyIC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SVISW::VALUE3)
    }
    #[doc = "HRPWMx.SyID"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SVISW::VALUE4)
    }
    #[doc = "HRPWMx.SyIE"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(SVISW::VALUE5)
    }
    #[doc = "HRPWMx.SyIF"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(SVISW::VALUE6)
    }
    #[doc = "HRPWMx.SyIG"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(SVISW::VALUE7)
    }
    #[doc = "HRPWMx.SyIH"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(SVISW::VALUE8)
    }
    #[doc = "HRPWMx.SyII"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(SVISW::VALUE9)
    }
    #[doc = "HRPWMx.SyIJ"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(SVISW::VALUE10)
    }
    #[doc = "HRPWMx.SyIK"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(SVISW::VALUE11)
    }
    #[doc = "HRPWMx.SyIL"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(SVISW::VALUE12)
    }
    #[doc = "HRPWMx.SyIM"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(SVISW::VALUE13)
    }
    #[doc = "HRPWMx.SyIN"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(SVISW::VALUE14)
    }
    #[doc = "HRPWMx.SyIO"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(SVISW::VALUE15)
    }
    #[doc = "HRPWMx.SyIP"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(SVISW::VALUE16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STRISW<'a> {
    w: &'a mut W,
}
impl<'a> _STRISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STPISW<'a> {
    w: &'a mut W,
}
impl<'a> _STPISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRGISW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STISW<'a> {
    w: &'a mut W,
}
impl<'a> _STISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCS`"]
pub enum SCSW {
    #[doc = "HRPWMx.MCLK (Module clock is used)"]
    VALUE1,
    #[doc = "HRPWMx.ECLKA (External clock is used)"]
    VALUE2,
    #[doc = "HRPWMx.ECLKB (External clock is used)"]
    VALUE3,
    #[doc = "HRPWMx.ECLKC (External clock is used)"]
    VALUE4,
}
impl SCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCSW::VALUE1 => 0,
            SCSW::VALUE2 => 1,
            SCSW::VALUE3 => 2,
            SCSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCSW<'a> {
    w: &'a mut W,
}
impl<'a> _SCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HRPWMx.MCLK (Module clock is used)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCSW::VALUE1)
    }
    #[doc = "HRPWMx.ECLKA (External clock is used)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCSW::VALUE2)
    }
    #[doc = "HRPWMx.ECLKB (External clock is used)"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCSW::VALUE3)
    }
    #[doc = "HRPWMx.ECLKC (External clock is used)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:3 - Value Selector input selection"]
    #[inline]
    pub fn svis(&self) -> SVISR {
        SVISR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Slope generation start control input selection"]
    #[inline]
    pub fn stris(&self) -> STRISR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STRISR { bits }
    }
    #[doc = "Bits 8:11 - Slope generation stop control input selection"]
    #[inline]
    pub fn stpis(&self) -> STPISR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STPISR { bits }
    }
    #[doc = "Bits 12:15 - External conversion trigger input selection"]
    #[inline]
    pub fn trgis(&self) -> TRGISR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGISR { bits }
    }
    #[doc = "Bits 16:19 - External shadow request enable input selection"]
    #[inline]
    pub fn stis(&self) -> STISR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STISR { bits }
    }
    #[doc = "Bits 20:21 - Slope generation clock selection"]
    #[inline]
    pub fn scs(&self) -> SCSR {
        SCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:3 - Value Selector input selection"]
    #[inline]
    pub fn svis(&mut self) -> _SVISW {
        _SVISW { w: self }
    }
    #[doc = "Bits 4:7 - Slope generation start control input selection"]
    #[inline]
    pub fn stris(&mut self) -> _STRISW {
        _STRISW { w: self }
    }
    #[doc = "Bits 8:11 - Slope generation stop control input selection"]
    #[inline]
    pub fn stpis(&mut self) -> _STPISW {
        _STPISW { w: self }
    }
    #[doc = "Bits 12:15 - External conversion trigger input selection"]
    #[inline]
    pub fn trgis(&mut self) -> _TRGISW {
        _TRGISW { w: self }
    }
    #[doc = "Bits 16:19 - External shadow request enable input selection"]
    #[inline]
    pub fn stis(&mut self) -> _STISW {
        _STISW { w: self }
    }
    #[doc = "Bits 20:21 - Slope generation clock selection"]
    #[inline]
    pub fn scs(&mut self) -> _SCSW {
        _SCSW { w: self }
    }
}
