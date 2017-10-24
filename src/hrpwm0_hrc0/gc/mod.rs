#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GC {
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
#[doc = "Possible values of the field `HRM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRM0R {
    #[doc = "Rising edge high resolution signal positioning enabled"]
    VALUE1,
    #[doc = "Falling edge high resolution signal positioning enabled"]
    VALUE2,
    #[doc = "Both edges high resolution signal positioning is enabled"]
    VALUE3,
    #[doc = "No high resolution positioning"]
    VALUE4,
}
impl HRM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HRM0R::VALUE1 => 0,
            HRM0R::VALUE2 => 1,
            HRM0R::VALUE3 => 2,
            HRM0R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HRM0R {
        match value {
            0 => HRM0R::VALUE1,
            1 => HRM0R::VALUE2,
            2 => HRM0R::VALUE3,
            3 => HRM0R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRM0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRM0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HRM0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == HRM0R::VALUE4
    }
}
#[doc = "Possible values of the field `HRM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRM1R {
    #[doc = "Rising edge high resolution signal positioning enabled"]
    VALUE1,
    #[doc = "Falling edge high resolution signal positioning enabled"]
    VALUE2,
    #[doc = "Both edges high resolution signal positioning is enabled"]
    VALUE3,
    #[doc = "No high resolution positioning"]
    VALUE4,
}
impl HRM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HRM1R::VALUE1 => 0,
            HRM1R::VALUE2 => 1,
            HRM1R::VALUE3 => 2,
            HRM1R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HRM1R {
        match value {
            0 => HRM1R::VALUE1,
            1 => HRM1R::VALUE2,
            2 => HRM1R::VALUE3,
            3 => HRM1R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HRM1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HRM1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HRM1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == HRM1R::VALUE4
    }
}
#[doc = "Possible values of the field `DTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTER {
    #[doc = "Dead time insertion is disabled"]
    VALUE1,
    #[doc = "Dead time insertion is enabled"]
    VALUE2,
}
impl DTER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DTER::VALUE1 => false,
            DTER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTER {
        match value {
            false => DTER::VALUE1,
            true => DTER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DTER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DTER::VALUE2
    }
}
#[doc = "Possible values of the field `TR0E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TR0ER {
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    VALUE1,
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    VALUE2,
}
impl TR0ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TR0ER::VALUE1 => false,
            TR0ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TR0ER {
        match value {
            false => TR0ER::VALUE1,
            true => TR0ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TR0ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TR0ER::VALUE2
    }
}
#[doc = "Possible values of the field `TR1E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TR1ER {
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    VALUE1,
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    VALUE2,
}
impl TR1ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TR1ER::VALUE1 => false,
            TR1ER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TR1ER {
        match value {
            false => TR1ER::VALUE1,
            true => TR1ER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TR1ER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TR1ER::VALUE2
    }
}
#[doc = "Possible values of the field `STC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STCR {
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    VALUE1,
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    VALUE2,
}
impl STCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STCR::VALUE1 => false,
            STCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STCR {
        match value {
            false => STCR::VALUE1,
            true => STCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STCR::VALUE2
    }
}
#[doc = "Possible values of the field `DSTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTCR {
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    VALUE1,
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    VALUE2,
}
impl DSTCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DSTCR::VALUE1 => false,
            DSTCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSTCR {
        match value {
            false => DSTCR::VALUE1,
            true => DSTCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DSTCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DSTCR::VALUE2
    }
}
#[doc = "Possible values of the field `OCS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS0R {
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    VALUE1,
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    VALUE2,
}
impl OCS0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OCS0R::VALUE1 => false,
            OCS0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCS0R {
        match value {
            false => OCS0R::VALUE1,
            true => OCS0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OCS0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OCS0R::VALUE2
    }
}
#[doc = "Possible values of the field `OCS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS1R {
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    VALUE1,
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    VALUE2,
}
impl OCS1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OCS1R::VALUE1 => false,
            OCS1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCS1R {
        match value {
            false => OCS1R::VALUE1,
            true => OCS1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OCS1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OCS1R::VALUE2
    }
}
#[doc = "Possible values of the field `DTUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTUSR {
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    VALUE1,
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    VALUE2,
}
impl DTUSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DTUSR::VALUE1 => false,
            DTUSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTUSR {
        match value {
            false => DTUSR::VALUE1,
            true => DTUSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DTUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DTUSR::VALUE2
    }
}
#[doc = "Values that can be written to the field `HRM0`"]
pub enum HRM0W {
    #[doc = "Rising edge high resolution signal positioning enabled"]
    VALUE1,
    #[doc = "Falling edge high resolution signal positioning enabled"]
    VALUE2,
    #[doc = "Both edges high resolution signal positioning is enabled"]
    VALUE3,
    #[doc = "No high resolution positioning"]
    VALUE4,
}
impl HRM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HRM0W::VALUE1 => 0,
            HRM0W::VALUE2 => 1,
            HRM0W::VALUE3 => 2,
            HRM0W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRM0W<'a> {
    w: &'a mut W,
}
impl<'a> _HRM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRM0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRM0W::VALUE1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRM0W::VALUE2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HRM0W::VALUE3)
    }
    #[doc = "No high resolution positioning"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(HRM0W::VALUE4)
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
#[doc = "Values that can be written to the field `HRM1`"]
pub enum HRM1W {
    #[doc = "Rising edge high resolution signal positioning enabled"]
    VALUE1,
    #[doc = "Falling edge high resolution signal positioning enabled"]
    VALUE2,
    #[doc = "Both edges high resolution signal positioning is enabled"]
    VALUE3,
    #[doc = "No high resolution positioning"]
    VALUE4,
}
impl HRM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HRM1W::VALUE1 => 0,
            HRM1W::VALUE2 => 1,
            HRM1W::VALUE3 => 2,
            HRM1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRM1W<'a> {
    w: &'a mut W,
}
impl<'a> _HRM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRM1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRM1W::VALUE1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRM1W::VALUE2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HRM1W::VALUE3)
    }
    #[doc = "No high resolution positioning"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(HRM1W::VALUE4)
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
#[doc = "Values that can be written to the field `DTE`"]
pub enum DTEW {
    #[doc = "Dead time insertion is disabled"]
    VALUE1,
    #[doc = "Dead time insertion is enabled"]
    VALUE2,
}
impl DTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTEW::VALUE1 => false,
            DTEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTEW<'a> {
    w: &'a mut W,
}
impl<'a> _DTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dead time insertion is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTEW::VALUE1)
    }
    #[doc = "Dead time insertion is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTEW::VALUE2)
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
#[doc = "Values that can be written to the field `TR0E`"]
pub enum TR0EW {
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    VALUE1,
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    VALUE2,
}
impl TR0EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TR0EW::VALUE1 => false,
            TR0EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TR0EW<'a> {
    w: &'a mut W,
}
impl<'a> _TR0EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TR0EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TR0EW::VALUE1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TR0EW::VALUE2)
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
#[doc = "Values that can be written to the field `TR1E`"]
pub enum TR1EW {
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    VALUE1,
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    VALUE2,
}
impl TR1EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TR1EW::VALUE1 => false,
            TR1EW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TR1EW<'a> {
    w: &'a mut W,
}
impl<'a> _TR1EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TR1EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TR1EW::VALUE1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TR1EW::VALUE2)
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
#[doc = "Values that can be written to the field `STC`"]
pub enum STCW {
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    VALUE1,
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    VALUE2,
}
impl STCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STCW::VALUE1 => false,
            STCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STCW<'a> {
    w: &'a mut W,
}
impl<'a> _STCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STCW::VALUE1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STCW::VALUE2)
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
#[doc = "Values that can be written to the field `DSTC`"]
pub enum DSTCW {
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    VALUE1,
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    VALUE2,
}
impl DSTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSTCW::VALUE1 => false,
            DSTCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSTCW::VALUE1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSTCW::VALUE2)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OCS0`"]
pub enum OCS0W {
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    VALUE1,
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    VALUE2,
}
impl OCS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCS0W::VALUE1 => false,
            OCS0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCS0W<'a> {
    w: &'a mut W,
}
impl<'a> _OCS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS0W::VALUE1)
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS0W::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OCS1`"]
pub enum OCS1W {
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    VALUE1,
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    VALUE2,
}
impl OCS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCS1W::VALUE1 => false,
            OCS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCS1W<'a> {
    w: &'a mut W,
}
impl<'a> _OCS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS1W::VALUE1)
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS1W::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTUS`"]
pub enum DTUSW {
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    VALUE1,
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    VALUE2,
}
impl DTUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTUSW::VALUE1 => false,
            DTUSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTUSW::VALUE1)
    }
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTUSW::VALUE2)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline]
    pub fn hrm0(&self) -> HRM0R {
        HRM0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline]
    pub fn hrm1(&self) -> HRM1R {
        HRM1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline]
    pub fn dte(&self) -> DTER {
        DTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline]
    pub fn tr0e(&self) -> TR0ER {
        TR0ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline]
    pub fn tr1e(&self) -> TR1ER {
        TR1ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline]
    pub fn stc(&self) -> STCR {
        STCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline]
    pub fn dstc(&self) -> DSTCR {
        DSTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline]
    pub fn ocs0(&self) -> OCS0R {
        OCS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline]
    pub fn ocs1(&self) -> OCS1R {
        OCS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline]
    pub fn dtus(&self) -> DTUSR {
        DTUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline]
    pub fn hrm0(&mut self) -> _HRM0W {
        _HRM0W { w: self }
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline]
    pub fn hrm1(&mut self) -> _HRM1W {
        _HRM1W { w: self }
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline]
    pub fn dte(&mut self) -> _DTEW {
        _DTEW { w: self }
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline]
    pub fn tr0e(&mut self) -> _TR0EW {
        _TR0EW { w: self }
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline]
    pub fn tr1e(&mut self) -> _TR1EW {
        _TR1EW { w: self }
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline]
    pub fn stc(&mut self) -> _STCW {
        _STCW { w: self }
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline]
    pub fn dstc(&mut self) -> _DSTCW {
        _DSTCW { w: self }
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline]
    pub fn ocs0(&mut self) -> _OCS0W {
        _OCS0W { w: self }
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline]
    pub fn ocs1(&mut self) -> _OCS1W {
        _OCS1W { w: self }
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline]
    pub fn dtus(&mut self) -> _DTUSW {
        _DTUSW { w: self }
    }
}
