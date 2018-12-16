#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRS {
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
#[doc = "Possible values of the field `VLS1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS1SR {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl VLS1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VLS1SR::VALUE1 => 0,
            VLS1SR::VALUE2 => 1,
            VLS1SR::VALUE3 => 2,
            VLS1SR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VLS1SR {
        match value {
            0 => VLS1SR::VALUE1,
            1 => VLS1SR::VALUE2,
            2 => VLS1SR::VALUE3,
            3 => VLS1SR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VLS1SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VLS1SR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == VLS1SR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == VLS1SR::VALUE4
    }
}
#[doc = "Possible values of the field `VLS2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS2SR {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl VLS2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VLS2SR::VALUE1 => 0,
            VLS2SR::VALUE2 => 1,
            VLS2SR::VALUE3 => 2,
            VLS2SR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VLS2SR {
        match value {
            0 => VLS2SR::VALUE1,
            1 => VLS2SR::VALUE2,
            2 => VLS2SR::VALUE3,
            3 => VLS2SR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VLS2SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VLS2SR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == VLS2SR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == VLS2SR::VALUE4
    }
}
#[doc = "Possible values of the field `TRLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRLSR {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl TRLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRLSR::VALUE1 => 0,
            TRLSR::VALUE2 => 1,
            TRLSR::VALUE3 => 2,
            TRLSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRLSR {
        match value {
            0 => TRLSR::VALUE1,
            1 => TRLSR::VALUE2,
            2 => TRLSR::VALUE3,
            3 => TRLSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TRLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TRLSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TRLSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TRLSR::VALUE4
    }
}
#[doc = "Possible values of the field `SSLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSLSR {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl SSLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSLSR::VALUE1 => 0,
            SSLSR::VALUE2 => 1,
            SSLSR::VALUE3 => 2,
            SSLSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSLSR {
        match value {
            0 => SSLSR::VALUE1,
            1 => SSLSR::VALUE2,
            2 => SSLSR::VALUE3,
            3 => SSLSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SSLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SSLSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SSLSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SSLSR::VALUE4
    }
}
#[doc = "Possible values of the field `STLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STLSR {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl STLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STLSR::VALUE1 => 0,
            STLSR::VALUE2 => 1,
            STLSR::VALUE3 => 2,
            STLSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STLSR {
        match value {
            0 => STLSR::VALUE1,
            1 => STLSR::VALUE2,
            2 => STLSR::VALUE3,
            3 => STLSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STLSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == STLSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == STLSR::VALUE4
    }
}
#[doc = "Possible values of the field `CRFLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRFLSR {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl CRFLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRFLSR::VALUE1 => 0,
            CRFLSR::VALUE2 => 1,
            CRFLSR::VALUE3 => 2,
            CRFLSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRFLSR {
        match value {
            0 => CRFLSR::VALUE1,
            1 => CRFLSR::VALUE2,
            2 => CRFLSR::VALUE3,
            3 => CRFLSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CRFLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CRFLSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CRFLSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CRFLSR::VALUE4
    }
}
#[doc = "Possible values of the field `CSLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSLSR {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl CSLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSLSR::VALUE1 => 0,
            CSLSR::VALUE2 => 1,
            CSLSR::VALUE3 => 2,
            CSLSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSLSR {
        match value {
            0 => CSLSR::VALUE1,
            1 => CSLSR::VALUE2,
            2 => CSLSR::VALUE3,
            3 => CSLSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CSLSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CSLSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CSLSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CSLSR::VALUE4
    }
}
#[doc = "Values that can be written to the field `VLS1S`"]
pub enum VLS1SW {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl VLS1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VLS1SW::VALUE1 => 0,
            VLS1SW::VALUE2 => 1,
            VLS1SW::VALUE3 => 2,
            VLS1SW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLS1SW<'a> {
    w: &'a mut W,
}
impl<'a> _VLS1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLS1SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VLS1SW::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VLS1SW::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(VLS1SW::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(VLS1SW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VLS2S`"]
pub enum VLS2SW {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl VLS2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VLS2SW::VALUE1 => 0,
            VLS2SW::VALUE2 => 1,
            VLS2SW::VALUE3 => 2,
            VLS2SW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLS2SW<'a> {
    w: &'a mut W,
}
impl<'a> _VLS2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLS2SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VLS2SW::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VLS2SW::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(VLS2SW::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(VLS2SW::VALUE4)
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
#[doc = "Values that can be written to the field `TRLS`"]
pub enum TRLSW {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl TRLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRLSW::VALUE1 => 0,
            TRLSW::VALUE2 => 1,
            TRLSW::VALUE3 => 2,
            TRLSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRLSW<'a> {
    w: &'a mut W,
}
impl<'a> _TRLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRLSW::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRLSW::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRLSW::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRLSW::VALUE4)
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
#[doc = "Values that can be written to the field `SSLS`"]
pub enum SSLSW {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl SSLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSLSW::VALUE1 => 0,
            SSLSW::VALUE2 => 1,
            SSLSW::VALUE3 => 2,
            SSLSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSLSW<'a> {
    w: &'a mut W,
}
impl<'a> _SSLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSLSW::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSLSW::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSLSW::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SSLSW::VALUE4)
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
#[doc = "Values that can be written to the field `STLS`"]
pub enum STLSW {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl STLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STLSW::VALUE1 => 0,
            STLSW::VALUE2 => 1,
            STLSW::VALUE3 => 2,
            STLSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STLSW<'a> {
    w: &'a mut W,
}
impl<'a> _STLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STLSW::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STLSW::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(STLSW::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(STLSW::VALUE4)
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
#[doc = "Values that can be written to the field `CRFLS`"]
pub enum CRFLSW {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl CRFLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRFLSW::VALUE1 => 0,
            CRFLSW::VALUE2 => 1,
            CRFLSW::VALUE3 => 2,
            CRFLSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRFLSW<'a> {
    w: &'a mut W,
}
impl<'a> _CRFLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRFLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRFLSW::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRFLSW::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CRFLSW::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CRFLSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSLS`"]
pub enum CSLSW {
    #[doc = "CSGySR0"]
    VALUE1,
    #[doc = "CSGySR1"]
    VALUE2,
    #[doc = "CSGySR2"]
    VALUE3,
    #[doc = "CSGySR3"]
    VALUE4,
}
impl CSLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSLSW::VALUE1 => 0,
            CSLSW::VALUE2 => 1,
            CSLSW::VALUE3 => 2,
            CSLSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSLSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGySR0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSLSW::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSLSW::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CSLSW::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CSLSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline]
    pub fn vls1s(&self) -> VLS1SR {
        VLS1SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline]
    pub fn vls2s(&self) -> VLS2SR {
        VLS2SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline]
    pub fn trls(&self) -> TRLSR {
        TRLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline]
    pub fn ssls(&self) -> SSLSR {
        SSLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline]
    pub fn stls(&self) -> STLSR {
        STLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline]
    pub fn crfls(&self) -> CRFLSR {
        CRFLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline]
    pub fn csls(&self) -> CSLSR {
        CSLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline]
    pub fn vls1s(&mut self) -> _VLS1SW {
        _VLS1SW { w: self }
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline]
    pub fn vls2s(&mut self) -> _VLS2SW {
        _VLS2SW { w: self }
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline]
    pub fn trls(&mut self) -> _TRLSW {
        _TRLSW { w: self }
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline]
    pub fn ssls(&mut self) -> _SSLSW {
        _SSLSW { w: self }
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline]
    pub fn stls(&mut self) -> _STLSW {
        _STLSW { w: self }
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline]
    pub fn crfls(&mut self) -> _CRFLSW {
        _CRFLSW { w: self }
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline]
    pub fn csls(&mut self) -> _CSLSW {
        _CSLSW { w: self }
    }
}
